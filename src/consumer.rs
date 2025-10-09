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
    ember::EmberPacket,
    error::EmberResult,
    glow::{Root, TreeNode},
};
use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpStream, sync::mpsc};
use tracing::info;

pub struct EmberConsumer {
    tx: mpsc::Sender<EmberPacket>,
    rx: mpsc::Receiver<EmberPacket>,
}

impl EmberConsumer {
    fn recursive_get_directory<'a>(
        &self,
        path: &[u32],
        node: TreeNode<'a>,
        consumer: impl Fn(&[u32], TreeNode),
    ) {
        match node {
            TreeNode::Node(node) => todo!(),
            TreeNode::QualifiedNode(qualified_node) => todo!(),
            TreeNode::Matrix(matrix) => todo!(),
            TreeNode::QualifiedMatrix(qualified_matrix) => todo!(),
            TreeNode::Parameter(parameter) => todo!(),
            TreeNode::QualifiedParameter(qualified_parameter) => todo!(),
            TreeNode::Template(template) => todo!(),
            TreeNode::QualifiedTemplate(qualified_template) => todo!(),
        }
    }
}

pub async fn start_tcp_consumer(
    provider_addr: SocketAddr,
    keepalive: Option<Duration>,
    try_use_non_escaping: bool,
) -> EmberResult<(mpsc::Sender<Root>, mpsc::Receiver<Root>)> {
    info!("Connecting to provider {provider_addr} …");

    let socket = TcpStream::connect(provider_addr).await?;
    socket.set_nodelay(true)?;

    info!("Successfully connected.");

    ember_client_channel(keepalive, socket, try_use_non_escaping).await
}
