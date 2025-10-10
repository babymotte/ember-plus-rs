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
        };

        spawn(async move {
            let cancel = consumer.shutdown_token.clone();
            if let Err(e) = consumer.run(api_rx).await {
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
                Some(msg) = self.ember_receiver.recv() => self.process_ember_message(msg).await?,
                _ = self.shutdown_token.cancelled() => break,
                else => break,
            }
        }

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

    async fn process_ember_message(&mut self, msg: Root) -> EmberResult<()> {
        trace!("Received ember message: {msg:?}");

        match msg {
            Root::Elements(root_element_collection) => {
                let mut used_recursive_fetch_callbacks = HashSet::new();
                for e in root_element_collection.0 {
                    match e {
                        TaggedRootElement(RootElement::Element(element)) => match element {
                            Element::Parameter(parameter) => {
                                self.process_unqualified_root_element(
                                    TreeNode::Parameter(parameter),
                                    &mut used_recursive_fetch_callbacks,
                                )
                                .await?;
                            }
                            Element::Node(node) => {
                                self.process_unqualified_root_element(
                                    TreeNode::Node(node),
                                    &mut used_recursive_fetch_callbacks,
                                )
                                .await?;
                            }
                            Element::Command(command) => {
                                // TODO can a producer send commands to a consumer?
                                warn!("Received command from producer: {command:?}");
                            }
                            Element::Matrix(matrix) => {
                                self.process_unqualified_root_element(
                                    TreeNode::Matrix(matrix),
                                    &mut used_recursive_fetch_callbacks,
                                )
                                .await?;
                            }
                            Element::Function(function) => {
                                // TODO can a producer send functions to a consumer?
                                warn!("Received function from producer: {function:?}");
                            }
                            Element::Template(template) => {
                                self.process_unqualified_root_element(
                                    TreeNode::Template(template),
                                    &mut used_recursive_fetch_callbacks,
                                )
                                .await?;
                            }
                        },
                        TaggedRootElement(RootElement::QualifiedParameter(qualified_parameter)) => {
                            let qulified_path = qualified_parameter.path.clone();
                            self.process_qualified_root_element(
                                qulified_path,
                                TreeNode::QualifiedParameter(qualified_parameter),
                                &mut used_recursive_fetch_callbacks,
                            )
                            .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedNode(qualified_node)) => {
                            let qulified_path = qualified_node.path.clone();
                            self.process_qualified_root_element(
                                qulified_path,
                                TreeNode::QualifiedNode(qualified_node),
                                &mut used_recursive_fetch_callbacks,
                            )
                            .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedMatrix(qualified_matrix)) => {
                            let qulified_path = qualified_matrix.path.clone();
                            self.process_qualified_root_element(
                                qulified_path,
                                TreeNode::QualifiedMatrix(qualified_matrix),
                                &mut used_recursive_fetch_callbacks,
                            )
                            .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedFunction(qualified_function)) => {
                            // TODO can a producer send functions to a consumer?
                            warn!(
                                "Received qualified function from producer: {qualified_function:?}"
                            );
                        }
                        TaggedRootElement(RootElement::QualifiedTemplate(qualified_template)) => {
                            let qulified_path = qualified_template.path.clone();
                            self.process_qualified_root_element(
                                qulified_path,
                                TreeNode::QualifiedTemplate(qualified_template),
                                &mut used_recursive_fetch_callbacks,
                            )
                            .await?;
                        }
                    }
                }
                for oid in used_recursive_fetch_callbacks {
                    self.recursive_fetch_callbacks.remove(&oid);
                    debug!("Removed recursive fetch callback for {oid}");
                }
            }
            Root::Streams(stream_collection) => todo!(),
            Root::InvocationResult(invocation_result) => todo!(),
        }

        Ok(())
    }

    async fn process_qualified_root_element(
        &mut self,
        qualified_path: RelativeOid,
        node: TreeNode,
        used_recursive_fetch_callbacks: &mut HashSet<RelativeOid>,
    ) -> Result<(), crate::error::EmberError> {
        let parent = qualified_path.parent();
        self.process_root_element(parent, node, used_recursive_fetch_callbacks)
            .await
    }

    async fn process_unqualified_root_element(
        &mut self,
        node: TreeNode,
        used_recursive_fetch_callbacks: &mut HashSet<RelativeOid>,
    ) -> Result<(), crate::error::EmberError> {
        let parent = RelativeOid::root();
        self.process_root_element(parent, node, used_recursive_fetch_callbacks)
            .await
    }

    async fn process_root_element(
        &mut self,
        parent: RelativeOid,
        node: TreeNode,
        used_recursive_fetch_callbacks: &mut HashSet<RelativeOid>,
    ) -> Result<(), crate::error::EmberError> {
        let recursive_fetch_callback = self.recursive_fetch_callbacks.get(&parent).cloned();
        used_recursive_fetch_callbacks.insert(parent.clone());
        self.process_ember_node(parent, node, recursive_fetch_callback.as_ref())
            .await?;
        Ok(())
    }

    async fn process_ember_node(
        &mut self,
        parent: RelativeOid,
        node: TreeNode,
        recursive_fetch_callback: Option<&mpsc::Sender<(RelativeOid, TreeNode)>>,
    ) -> EmberResult<bool> {
        let oid = node.oid(&parent);

        debug!("Got content of node {parent}: {oid} {node:?}");

        // this applies to non-leaf nodes in a tree structure
        if let Some((path, children)) = node.clone().children(&parent) {
            debug!("Node {oid} seems to be a container, processing children …");
            // TODO check if node is already known and treat it like a full node if it is not
            let recursive_fetch_callback = self.recursive_fetch_callbacks.get(&path).cloned();
            let mut remove_recursive_fetch_callback = true;
            for node in children {
                debug!("Processing child of {path}: {}", node.oid(&path));
                remove_recursive_fetch_callback &= Box::pin(self.process_ember_node(
                    path.clone(),
                    node,
                    recursive_fetch_callback.as_ref(),
                ))
                .await?;
            }
            if remove_recursive_fetch_callback {
                debug!("Removing recursive fetch callback for node {parent}.");
                self.recursive_fetch_callbacks.remove(&path);
            }
            Ok(false)
        }
        // this applies to leaf nodes in a tree structure or to qualified nodes
        else {
            debug!("Looking up callbacks for node {oid} …");

            if let Some(callback) = self.permanent_callbacks.get(&parent) {
                debug!("Found callback for node {oid}");
                callback.send((parent.clone(), node.clone())).await.ok();
            }

            if let Some(callback) = recursive_fetch_callback {
                debug!("Found recursive fetch callback for node {oid}");
                callback.send((parent.clone(), node.clone())).await.ok();
            }

            Ok(true)
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
        debug!("Fetching content of node {oid}: {node:?} using request: {request:?}");
        let api = self.api.clone();

        self.permanent_callbacks
            .insert(oid.clone(), consumer.clone());
        let (tx, mut rx) = mpsc::channel(1024);
        spawn(async move {
            while let Some((p, n)) = rx.recv().await {
                api.fetch_recursive(p, n, consumer.clone()).await;
            }
        });
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
    debug!("Connecting to provider {provider_addr} …");

    let socket = TcpStream::connect(provider_addr).await?;
    socket.set_nodelay(true)?;

    debug!("Successfully connected.");

    let (tx, rx) = ember_client_channel(keepalive, socket, try_use_non_escaping).await?;

    let api = EmberConsumer::start(tx, rx, cancellation_token);

    Ok(api)
}
