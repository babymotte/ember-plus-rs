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
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    time::Duration,
};
use tokio::{net::TcpStream, select, spawn, sync::mpsc};
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tracing")]
use tracing::{debug, error, trace, warn};

pub type NodeCallback = mpsc::Sender<(RelativeOid, TreeNode)>;

pub enum EmberConsumerApiMessage {
    FetchRecursive(RelativeOid, TreeNode, NodeCallback),
}

#[derive(Clone)]
pub struct EmberConsumerApi {
    tx: mpsc::Sender<EmberConsumerApiMessage>,
}

impl EmberConsumerApi {
    pub async fn fetch_full_tree(&self) -> mpsc::Receiver<(RelativeOid, TreeNode)> {
        let (tx, rx) = mpsc::channel(1024);
        self.fetch_recursive(RelativeOid::root(), TreeNode::Root, tx)
            .await;
        rx
    }

    pub async fn fetch_recursive(
        &self,
        parent: RelativeOid,
        node: TreeNode,
        consumer: NodeCallback,
    ) {
        self.tx
            .send(EmberConsumerApiMessage::FetchRecursive(
                parent, node, consumer,
            ))
            .await
            .ok();
    }
}

pub struct EmberConsumer {
    ember_sender: mpsc::Sender<Root>,
    ember_receiver: mpsc::Receiver<Root>,
    permanent_callbacks: HashMap<RelativeOid, NodeCallback>,
    recursive_fetch_callbacks: HashMap<RelativeOid, NodeCallback>,
    shutdown_token: CancellationToken,
    api: EmberConsumerApi,
    requested_directories: HashSet<RelativeOid>,
}

impl EmberConsumer {
    fn start(
        ember_sender: mpsc::Sender<Root>,
        ember_receiver: mpsc::Receiver<Root>,
        shutdown_token: CancellationToken,
    ) -> EmberConsumerApi {
        let (api_tx, api_rx) = mpsc::channel(1024);
        let api = EmberConsumerApi { tx: api_tx };

        let consumer = Self {
            ember_sender,
            ember_receiver,
            permanent_callbacks: HashMap::new(),
            recursive_fetch_callbacks: HashMap::new(),
            shutdown_token,
            api: api.clone(),
            requested_directories: HashSet::new(),
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
                Some(recv) = rx.recv() => self.process_api_message(recv).await?,
                Some(msg) = self.ember_receiver.recv() => if self.process_ember_message(msg).await? {
                    break;
                },
                _ = self.shutdown_token.cancelled() => break,
                else => break,
            }
        }

        error!("Ember consumer closed.");
        self.shutdown_token.cancel();

        Ok(())
    }

    async fn process_api_message(&mut self, msg: EmberConsumerApiMessage) -> EmberResult<()> {
        match msg {
            EmberConsumerApiMessage::FetchRecursive(parent, node, consumer) => {
                self.fetch_recursive(parent, node, consumer).await;
            }
        }
        Ok(())
    }

    async fn process_ember_message(&mut self, msg: Root) -> EmberResult<bool> {
        #[cfg(feature = "tracing")]
        trace!("Received ember message: {msg}");

        let mut cancel = false;

        match msg {
            Root::Elements(root_element_collection) => {
                for e in root_element_collection.0 {
                    match e {
                        TaggedRootElement(RootElement::Element(element)) => match element {
                            Element::Parameter(parameter) => {
                                cancel |= self
                                    .process_unqualified_root_element(TreeNode::Parameter(
                                        parameter,
                                    ))
                                    .await?;
                            }
                            Element::Node(node) => {
                                cancel |= self
                                    .process_unqualified_root_element(TreeNode::Node(node))
                                    .await?;
                            }
                            Element::Command(command) => {
                                // TODO can a producer send commands to a consumer?
                                #[cfg(feature = "tracing")]
                                warn!("Received command from producer: {command:?}");
                            }
                            Element::Matrix(matrix) => {
                                cancel |= self
                                    .process_unqualified_root_element(TreeNode::Matrix(matrix))
                                    .await?;
                            }
                            Element::Function(function) => {
                                // TODO can a producer send functions to a consumer?
                                #[cfg(feature = "tracing")]
                                warn!("Received function from producer: {function:?}");
                            }
                            Element::Template(template) => {
                                cancel |= self
                                    .process_unqualified_root_element(TreeNode::Template(template))
                                    .await?;
                            }
                        },
                        TaggedRootElement(RootElement::QualifiedParameter(qualified_parameter)) => {
                            let qulified_path = qualified_parameter.path.clone();
                            cancel |= self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedParameter(qualified_parameter),
                                )
                                .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedNode(qualified_node)) => {
                            let qulified_path = qualified_node.path.clone();
                            cancel |= self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedNode(qualified_node),
                                )
                                .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedMatrix(qualified_matrix)) => {
                            let qulified_path = qualified_matrix.path.clone();
                            cancel |= self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedMatrix(qualified_matrix),
                                )
                                .await?;
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
                            cancel |= self
                                .process_qualified_root_element(
                                    qulified_path,
                                    TreeNode::QualifiedTemplate(qualified_template),
                                )
                                .await?;
                        }
                    }
                }
            }
            Root::Streams(stream_collection) => todo!(),
            Root::InvocationResult(invocation_result) => todo!(),
        }

        Ok(cancel)
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
        let recursive_fetch_callback = self.recursive_fetch_callbacks.get(&parent).cloned();
        self.process_ember_node(parent, node, recursive_fetch_callback.as_ref())
            .await
    }

    async fn process_ember_node(
        &mut self,
        parent: RelativeOid,
        node: TreeNode,
        recursive_fetch_callback: Option<&mpsc::Sender<(RelativeOid, TreeNode)>>,
    ) -> EmberResult<bool> {
        let oid = node.oid(&parent);

        #[cfg(feature = "tracing")]
        debug!("Got content of node {parent}: {oid} {node}");

        // TODO detect unknown nodes and traverse them automatically

        // this applies to non-leaf nodes in a tree structure
        if let Some((path, children)) = node.clone().children(&parent) {
            #[cfg(feature = "tracing")]
            debug!("Node {oid} seems to be a container, processing children …");
            let recursive_fetch_callback = self.recursive_fetch_callbacks.get(&path).cloned();
            let mut cancel = false;
            for node in children {
                #[cfg(feature = "tracing")]
                debug!("Processing child of {path}: {}", node.oid(&path));
                cancel |= Box::pin(self.process_ember_node(
                    path.clone(),
                    node,
                    recursive_fetch_callback.as_ref(),
                ))
                .await?;
            }
            Ok(cancel)
        }
        // this applies to leaf nodes in a tree structure or to qualified nodes
        else {
            #[cfg(feature = "tracing")]
            debug!("Looking up callbacks for node {oid} …");

            if let Some(callback) = self.permanent_callbacks.get(&parent) {
                #[cfg(feature = "tracing")]
                debug!("Found callback for node {oid}");
                if callback.send((parent.clone(), node.clone())).await.is_err() {
                    return Ok(true);
                }
            }

            if let Some(callback) = recursive_fetch_callback {
                #[cfg(feature = "tracing")]
                debug!("Found recursive fetch callback for node {oid}");
                callback.send((parent.clone(), node.clone())).await.ok();
            }

            Ok(false)
        }
    }

    async fn fetch_recursive(
        &mut self,
        parent: RelativeOid,
        node: TreeNode,
        consumer: NodeCallback,
    ) {
        let Some((oid, request)) = node.clone().get_directory(&parent) else {
            return;
        };

        if !self.requested_directories.insert(oid.clone()) {
            #[cfg(feature = "tracing")]
            debug!("Content of node {oid} already requested.");
            return;
        }

        #[cfg(feature = "tracing")]
        debug!("Fetching content of node {oid}: {node} using request: {request}");
        let api = self.api.clone();

        self.permanent_callbacks
            .insert(oid.clone(), consumer.clone());
        let (tx, mut rx) = mpsc::channel(1024);
        spawn(async move {
            while let Some((p, n)) = rx.recv().await {
                api.fetch_recursive(p, n, consumer.clone()).await;
            }
        });
        #[cfg(feature = "tracing")]
        debug!("Adding recursive fetch callback for {oid} …");
        self.recursive_fetch_callbacks.insert(oid, tx);
        self.ember_sender.send(request).await.ok();
    }
}

pub async fn start_tcp_consumer(
    provider_addr: SocketAddr,
    keepalive: Option<Duration>,
    try_use_non_escaping: bool,
    cancellation_token: CancellationToken,
) -> EmberResult<EmberConsumerApi> {
    #[cfg(feature = "tracing")]
    debug!("Connecting to provider {provider_addr} …");

    let socket = TcpStream::connect(provider_addr).await?;
    socket.set_nodelay(true)?;

    #[cfg(feature = "tracing")]
    debug!("Successfully connected.");

    let (tx, rx) = ember_client_channel(keepalive, socket, try_use_non_escaping).await?;

    let api = EmberConsumer::start(tx, rx, cancellation_token);

    Ok(api)
}
