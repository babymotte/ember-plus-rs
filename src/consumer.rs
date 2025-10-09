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
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use tokio::{
    net::TcpStream,
    select, spawn,
    sync::{mpsc, oneshot},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, trace, warn};

pub type NodeCallback = mpsc::Sender<(Option<RelativeOid>, TreeNode)>;
pub type NodeOneshotCallback = oneshot::Sender<(Option<RelativeOid>, TreeNode)>;

pub enum EmberConsumerApiMessage {
    FetchRecursive(Option<RelativeOid>, TreeNode, NodeCallback),
}

#[derive(Clone)]
pub struct EmberConsumerApi {
    tx: mpsc::Sender<EmberConsumerApiMessage>,
}

impl EmberConsumerApi {
    pub async fn fetch_full_tree(&self) -> mpsc::Receiver<(Option<RelativeOid>, TreeNode)> {
        let (tx, rx) = mpsc::channel(1024);
        self.fetch_recursive(None, TreeNode::Empty, tx).await;
        rx
    }

    pub async fn fetch_recursive(
        &self,
        parent: Option<RelativeOid>,
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
    tx: mpsc::Sender<Root>,
    rx: mpsc::Receiver<Root>,
    callbacks: HashMap<Option<RelativeOid>, NodeCallback>,
    oneshot_callbacks: HashMap<Option<RelativeOid>, NodeOneshotCallback>,
    cancellation_token: CancellationToken,
    api: EmberConsumerApi,
}

impl EmberConsumer {
    fn start(
        tx: mpsc::Sender<Root>,
        rx: mpsc::Receiver<Root>,
        cancellation_token: CancellationToken,
    ) -> EmberConsumerApi {
        let (api_tx, api_rx) = mpsc::channel(1024);
        let api = EmberConsumerApi { tx: api_tx };

        let consumer = Self {
            tx,
            rx,
            callbacks: HashMap::new(),
            oneshot_callbacks: HashMap::new(),
            cancellation_token,
            api: api.clone(),
        };

        spawn(async move {
            let cancel = consumer.cancellation_token.clone();
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
                Some(msg) = self.rx.recv() => self.process_ember_message(msg).await?,
                _ = self.cancellation_token.cancelled() => break,
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
                for e in root_element_collection.0 {
                    match e {
                        TaggedRootElement(RootElement::Element(element)) => match element {
                            Element::Parameter(parameter) => {
                                self.process_ember_node(None, TreeNode::Parameter(parameter))
                                    .await?;
                            }
                            Element::Node(node) => {
                                self.process_ember_node(None, TreeNode::Node(node)).await?;
                            }
                            Element::Command(command) => {
                                // TODO can a producer send commands to a consumer?
                                warn!("Received command from producer: {command:?}");
                            }
                            Element::Matrix(matrix) => {
                                self.process_ember_node(None, TreeNode::Matrix(matrix))
                                    .await?;
                            }
                            Element::Function(function) => {
                                // TODO can a producer send functions to a consumer?
                                warn!("Received function from producer: {function:?}");
                            }
                            Element::Template(template) => {
                                self.process_ember_node(None, TreeNode::Template(template))
                                    .await?;
                            }
                        },
                        TaggedRootElement(RootElement::QualifiedParameter(qualified_parameter)) => {
                            self.process_ember_node(
                                None,
                                TreeNode::QualifiedParameter(qualified_parameter),
                            )
                            .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedNode(qualified_node)) => {
                            self.process_ember_node(None, TreeNode::QualifiedNode(qualified_node))
                                .await?;
                        }
                        TaggedRootElement(RootElement::QualifiedMatrix(qualified_matrix)) => {
                            self.process_ember_node(
                                None,
                                TreeNode::QualifiedMatrix(qualified_matrix),
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
                            self.process_ember_node(
                                None,
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

        Ok(())
    }

    async fn process_ember_node(
        &mut self,
        parent: Option<RelativeOid>,
        node: TreeNode,
    ) -> EmberResult<()> {
        trace!("Processing ember node: {node:?} (parent: {parent:?})");

        if let Some(callback) = self.callbacks.get(&parent) {
            trace!("Found callback for node {node:?}");
            callback.send((parent.clone(), node.clone())).await.ok();
        }

        if let Some(callback) = self.oneshot_callbacks.remove(&parent) {
            trace!("Found oneshot callback for node {node:?}");
            callback.send((parent.clone(), node.clone())).ok();
        }

        Ok(())
    }

    async fn fetch_recursive(
        &mut self,
        parent: Option<RelativeOid>,
        node: TreeNode,
        consumer: NodeCallback,
    ) {
        let Some(request) = node.clone().get_directory(parent.as_ref()) else {
            return;
        };
        debug!("fetching content of node {parent:?}: {node:?} using request: {request:?}");
        let api = self.api.clone();

        self.callbacks.insert(parent.clone(), consumer.clone());
        let (tx, rx) = oneshot::channel();
        spawn(async move {
            if let Ok((p, n)) = rx.await {
                api.fetch_recursive(p, n, consumer).await;
            }
        });
        self.oneshot_callbacks.insert(parent, tx);
        self.tx.send(request).await.ok();
    }
}

pub async fn start_tcp_consumer(
    provider_addr: SocketAddr,
    keepalive: Option<Duration>,
    try_use_non_escaping: bool,
    cancellation_token: CancellationToken,
) -> EmberResult<EmberConsumerApi> {
    info!("Connecting to provider {provider_addr} …");

    let socket = TcpStream::connect(provider_addr).await?;
    socket.set_nodelay(true)?;

    info!("Successfully connected.");

    let (tx, rx) = ember_client_channel(keepalive, socket, try_use_non_escaping).await?;

    let api = EmberConsumer::start(tx, rx, cancellation_token);

    Ok(api)
}
