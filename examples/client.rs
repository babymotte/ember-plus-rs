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

use ember_plus_rs::{
    consumer::start_tcp_consumer,
    error::EmberResult,
    glow::{Command, FieldFlags, Root, TreeNode},
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> EmberResult<()> {
    tracing_subscriber::fmt().init();

    let (tx, mut rx) = start_tcp_consumer(
        "127.0.0.1:9000".parse().expect("malformed socket address"),
        // Some(Duration::from_secs(1)),
        None,
        false,
    )
    .await?;

    let msg = Root::from(Command::get_directory(Some(FieldFlags::All)));

    tx.send(msg).await.ok();

    while let Some(msg) = rx.recv().await {
        info!("Received ember message: {msg:?}");
        // TODO
    }

    Ok(())
}

fn recursive_get_directory<'a>(
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
