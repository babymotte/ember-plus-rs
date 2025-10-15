/*
 *  Copyright (C) 2025 Michael Bachmann
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{
    com::ember_client_channel,
    error::EmberResult,
    glow::{Element, RelativeOid, Root, RootElement, TaggedRootElement, TreeNode},
};
use std::{collections::HashSet, net::SocketAddr, time::Duration};
use tokio::{net::TcpStream, select, spawn, sync::mpsc};
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tracing")]
use tracing::{debug, error, trace, warn};

pub type NodeConsumer = mpsc::Sender<TreeEvent>;

pub enum EmberConsumerApiMessage {
    FetchRecursive {
        parent: RelativeOid,
        node: TreeNode,
        consumer: NodeConsumer,
    },
}

#[derive(Clone)]
pub struct EmberConsumerApi {
    tx: mpsc::Sender<EmberConsumerApiMessage>,
}

pub enum TreeEvent {
    Element((RelativeOid, TreeNode)),
    FullTreeReceived(usize),
}

impl EmberConsumerApi {
    pub async fn fetch_full_tree(&self) -> mpsc::Receiver<TreeEvent> {
        let (tx, rx) = mpsc::channel(1024 * 1024);
        self.fetch_recursive(RelativeOid::root(), TreeNode::Root, tx)
            .await;
        rx
    }

    pub async fn fetch_recursive(
        &self,
        parent: RelativeOid,
        node: TreeNode,
        consumer: NodeConsumer,
    ) {
        self.tx
            .send(EmberConsumerApiMessage::FetchRecursive {
                parent,
                node,
                consumer,
            })
            .await
            .ok();
    }
}

pub struct EmberConsumer {
    ember_sender: mpsc::Sender<Root>,
    ember_receiver: mpsc::Receiver<Root>,
    permanent_consumers: Vec<NodeConsumer>,
    shutdown_token: CancellationToken,
    in_flight: HashSet<RelativeOid>,
    explored: HashSet<RelativeOid>,
    query_offline_nodes: bool,
}

impl EmberConsumer {
    fn start(
        ember_sender: mpsc::Sender<Root>,
        ember_receiver: mpsc::Receiver<Root>,
        shutdown_token: CancellationToken,
        query_offline_nodes: bool,
    ) -> EmberConsumerApi {
        let (api_tx, api_rx) = mpsc::channel(1024);
        let api = EmberConsumerApi { tx: api_tx };

        let consumer = Self {
            ember_sender,
            ember_receiver,
            permanent_consumers: Vec::new(),
            shutdown_token,
            in_flight: HashSet::new(),
            explored: HashSet::new(),
            query_offline_nodes,
        };

        spawn(async move {
            let cancel = consumer.shutdown_token.clone();
            if let Err(e) = consumer.run(api_rx).await {
                #[cfg(feature = "tracing")]
                error!("Error in Ember+ consumer: {e}");
                cancel.cancel();
            }
        });

        api
    }

    async fn run(mut self, mut rx: mpsc::Receiver<EmberConsumerApiMessage>) -> EmberResult<()> {
        loop {
            select! {
                Some(recv) = rx.recv() => if self.process_api_message(recv).await {
                    break;
                },
                Some(msg) = self.ember_receiver.recv() => if self.process_ember_message(msg).await? {
                    break;
                },
                _ = self.shutdown_token.cancelled() => break,
                else => break,
            }
        }

        #[cfg(feature = "tracing")]
        warn!("Ember consumer closed.");

        self.shutdown_token.cancel();

        Ok(())
    }

    #[must_use]
    async fn process_api_message(&mut self, msg: EmberConsumerApiMessage) -> bool {
        match msg {
            EmberConsumerApiMessage::FetchRecursive {
                parent,
                node,
                consumer,
            } => {
                self.permanent_consumers.push(consumer);
                self.fetch_recursive(parent, node).await
            }
        }
    }

    async fn process_ember_message(&mut self, msg: Root) -> EmberResult<bool> {
        #[cfg(feature = "tracing")]
        trace!("Received ember message: {msg}");

        match msg {
            Root::Elements(root_element_collection) => {
                for e in root_element_collection.0 {
                    match e {
                        TaggedRootElement(RootElement::Element(element)) => match element {
                            Element::Parameter(parameter) => {
                                if self
                                    .process_unqualified_root_element(TreeNode::Parameter(
                                        parameter,
                                    ))
                                    .await?
                                {
                                    return Ok(true);
                                }
                            }
                            Element::Node(node) => {
                                if self
                                    .process_unqualified_root_element(TreeNode::Node(node))
                                    .await?
                                {
                                    return Ok(true);
                                }
                            }
                            Element::Command(command) => {
                                // TODO can a producer send commands to a consumer?
                                #[cfg(feature = "tracing")]
                                warn!("Received command from producer: {command:?}");
                            }
                            Element::Matrix(matrix) => {
                                if self
                                    .process_unqualified_root_element(TreeNode::Matrix(matrix))
                                    .await?
                                {
                                    return Ok(true);
                                }
                            }
                            Element::Function(function) => {
                                // TODO can a producer send functions to a consumer?
                                #[cfg(feature = "tracing")]
                                warn!("Received function from producer: {function:?}");
                            }
                            Element::Template(template) => {
                                if self
                                    .process_unqualified_root_element(TreeNode::Template(template))
                                    .await?
                                {
                                    return Ok(true);
                                }
                            }
                        },
                        TaggedRootElement(RootElement::QualifiedParameter(qualified_parameter)) => {
                            let qulified_path = qualified_parameter.path.clone();
                            if self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedParameter(qualified_parameter),
                                )
                                .await?
                            {
                                return Ok(true);
                            }
                        }
                        TaggedRootElement(RootElement::QualifiedNode(qualified_node)) => {
                            let qulified_path = qualified_node.path.clone();
                            if self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedNode(qualified_node),
                                )
                                .await?
                            {
                                return Ok(true);
                            }
                        }
                        TaggedRootElement(RootElement::QualifiedMatrix(qualified_matrix)) => {
                            let qulified_path = qualified_matrix.path.clone();
                            if self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedMatrix(qualified_matrix),
                                )
                                .await?
                            {
                                return Ok(true);
                            }
                        }
                        TaggedRootElement(RootElement::QualifiedFunction(qualified_function)) => {
                            // TODO can a producer send functions to a consumer?
                            #[cfg(feature = "tracing")]
                            warn!(
                                "Received qualified function from producer: {qualified_function:?}"
                            );
                        }
                        TaggedRootElement(RootElement::QualifiedTemplate(qualified_template)) => {
                            let qulified_path = qualified_template.path.clone();
                            if self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedTemplate(qualified_template),
                                )
                                .await?
                            {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            Root::Streams(stream_collection) => todo!(),
            Root::InvocationResult(invocation_result) => todo!(),
        }

        Ok(false)
    }

    async fn process_qualified_root_element(
        &mut self,
        qualified_path: RelativeOid,
        node: TreeNode,
    ) -> EmberResult<bool> {
        let parent = qualified_path.parent();
        self.process_root_element(parent, node).await
    }

    async fn process_unqualified_root_element(&mut self, node: TreeNode) -> EmberResult<bool> {
        let parent = RelativeOid::root();
        self.process_root_element(parent, node).await
    }

    async fn process_root_element(
        &mut self,
        parent: RelativeOid,
        node: TreeNode,
    ) -> EmberResult<bool> {
        self.process_ember_node(parent, node).await
    }

    async fn process_ember_node(
        &mut self,
        parent: RelativeOid,
        node: TreeNode,
    ) -> EmberResult<bool> {
        let oid = node.oid(&parent);

        #[cfg(feature = "tracing")]
        debug!("Got content of node {parent}: {oid} {node}");

        // this applies to non-leaf nodes in a tree structure
        if let Some((path, children)) = node.clone().children(&parent) {
            #[cfg(feature = "tracing")]
            debug!("Node {oid} seems to be a container, processing children …");
            for node in children {
                #[cfg(feature = "tracing")]
                debug!("Processing child of {path}: {}", node.oid(&path));
                if Box::pin(self.process_ember_node(path.clone(), node)).await? {
                    return Ok(true);
                }
            }
        }
        // this applies to leaf nodes in a tree structure or to qualified nodes
        else {
            #[cfg(feature = "tracing")]
            debug!("Looking up callbacks for node {oid} …");

            if node.may_have_children() {
                if self.explored.insert(oid.clone()) {
                    let p = parent.clone();
                    let n = node.clone();
                    if self.fetch_recursive(p, n).await {
                        return Ok(true);
                    }
                } else {
                    #[cfg(feature = "tracing")]
                    debug!("Content of node {oid} already requested.");
                }
            }

            for consumer in &self.permanent_consumers {
                if consumer
                    .send(TreeEvent::Element((parent.clone(), node.clone())))
                    .await
                    .is_err()
                {
                    return Ok(true);
                }
            }
        }

        if self.in_flight.remove(&parent) {
            #[cfg(feature = "tracing")]
            debug!("In flight GET_DIRECTORY commands: {}", self.in_flight.len());

            if self.in_flight.is_empty() {
                for consumer in &self.permanent_consumers {
                    if consumer
                        .send(TreeEvent::FullTreeReceived(self.explored.len()))
                        .await
                        .is_err()
                    {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    #[must_use]
    async fn fetch_recursive(&mut self, parent: RelativeOid, node: TreeNode) -> bool {
        let Some((oid, request)) = node.clone().get_directory(&parent) else {
            return false;
        };

        if !node.is_online() && !self.query_offline_nodes {
            #[cfg(feature = "tracing")]
            warn!(
                "Not fetching content of node {} because it is offline.",
                oid
            );
            return false;
        }

        #[cfg(feature = "tracing")]
        debug!("Fetching content of node {oid}: {node} using request: {request}");

        self.in_flight.insert(oid.clone());
        #[cfg(feature = "tracing")]
        debug!("In flight GET_DIRECTORY commands: {}", self.in_flight.len());

        #[cfg(feature = "tracing")]
        if self.explored.len() % 1_000 == 0 {
            use tracing::info;
            info!("Requested content of {} nodes …", self.explored.len());
        }

        self.ember_sender.send(request).await.is_err()
    }
}

pub async fn start_tcp_consumer(
    provider_addr: SocketAddr,
    keepalive: Option<Duration>,
    try_use_non_escaping: bool,
    cancellation_token: CancellationToken,
    query_offline_nodes: bool,
) -> EmberResult<EmberConsumerApi> {
    #[cfg(feature = "tracing")]
    debug!("Connecting to provider {provider_addr} …");

    let socket = TcpStream::connect(provider_addr).await?;
    socket.set_nodelay(true)?;

    #[cfg(feature = "tracing")]
    debug!("Successfully connected.");

    let (tx, rx) = ember_client_channel(keepalive, socket, try_use_non_escaping).await?;

    let api = EmberConsumer::start(tx, rx, cancellation_token, query_offline_nodes);

    Ok(api)
}
