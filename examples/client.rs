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
    consumer::{TreeEvent, start_tcp_consumer},
    glow::{RelativeOid, TreeNode},
};
use miette::Result;
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::select;
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tracing")]
use tracing::{debug, info};
use worterbuch_client::{Value, Worterbuch, topic};

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "tracing")]
    logging::init();

    let (wb, _, _) = worterbuch_client::connect_with_default_config().await?;
    wb.set_grave_goods(&["ember/#"]).await?;

    let shutdown_token = CancellationToken::new();

    let consumer = start_tcp_consumer(
        "127.0.0.1:9000".parse().expect("malformed socket address"),
        Some(Duration::from_secs(1)),
        false,
        shutdown_token.clone(),
        false,
    )
    .await?;

    let start = Instant::now();

    #[cfg(feature = "tracing")]
    info!("Fetching tree …");

    let mut rx = consumer.fetch_full_tree().await;

    loop {
        select! {
            Some(ev) = rx.recv() => process_event(ev, &wb, start).await?,
            _ = shutdown_token.cancelled() => break,
            else => break,
        }
    }

    #[cfg(feature = "tracing")]
    info!("Client closed.");

    Ok(())
}

async fn process_event(event: TreeEvent, wb: &Worterbuch, start: Instant) -> Result<()> {
    match event {
        TreeEvent::Element(element) => process_tree_element(element.0, element.1, wb).await?,
        TreeEvent::FullTreeReceived(nodes) => {
            #[cfg(feature = "tracing")]
            info!(
                "Full tree with {} nodes received after {:?}",
                nodes,
                start.elapsed()
            );
        }
    }

    Ok(())
}

async fn process_tree_element(parent: RelativeOid, node: TreeNode, wb: &Worterbuch) -> Result<()> {
    let oid = node.oid(&parent);

    #[cfg(feature = "tracing")]
    debug!("Got update for content of node {parent}: {node}");

    match node {
        TreeNode::Node(node) => {
            if let Some(contents) = node.contents {
                publish(key(oid), json!(contents), wb).await?;
            }
        }
        TreeNode::QualifiedNode(node) => {
            if let Some(contents) = node.contents {
                publish(key(oid), json!(contents), wb).await?;
            }
        }
        TreeNode::Parameter(param) => {
            if let Some(contents) = param.contents {
                publish(key(oid), json!(contents), wb).await?;
            }
        }
        TreeNode::QualifiedParameter(param) => {
            if let Some(contents) = param.contents {
                publish(key(oid), json!(contents), wb).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn publish(key: String, value: Value, wb: &Worterbuch) -> Result<()> {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                Box::pin(publish(topic!(key, k), v, wb)).await?;
            }
        }
        val => {
            wb.set_async(key, val).await?;
        }
    }

    Ok(())
}

fn key(oid: RelativeOid) -> String {
    format!("ember{}", oid.to_string().replace(".", "/children/"))
}

#[cfg(feature = "tracing")]
mod logging {
    use std::io;
    use supports_color::Stream;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{
        EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt,
    };

    pub(crate) fn init() {
        tracing_subscriber::registry()
            .with(
                fmt::Layer::new()
                    .with_ansi(supports_color::on(Stream::Stderr).is_some())
                    .with_writer(io::stderr)
                    .with_filter(
                        EnvFilter::builder()
                            .with_default_directive(LevelFilter::INFO.into())
                            .with_env_var("EMBER_LOG")
                            .from_env_lossy(),
                    ),
            )
            .init();
    }
}
