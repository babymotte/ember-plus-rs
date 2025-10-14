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

use std::time::Instant;

use ember_plus_rs::{
    consumer::{TreeEvent, start_tcp_consumer},
    glow::{RelativeOid, TreeNode, Value},
};
use miette::Result;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::debug;
#[cfg(feature = "tracing")]
use tracing::info;
use worterbuch_client::Worterbuch;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "tracing")]
    logging::init();

    let (wb, _, _) = worterbuch_client::connect_with_default_config().await?;
    wb.set_grave_goods(&["ember/#"]).await?;

    let shutdown_token = CancellationToken::new();

    let consumer = start_tcp_consumer(
        "127.0.0.1:9000".parse().expect("malformed socket address"),
        // Some(Duration::from_secs(1)),
        None,
        false,
        shutdown_token.clone(),
    )
    .await?;

    let start = Instant::now();

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
        TreeEvent::Element((parent, node)) => process_tree_element(parent, node, wb).await?,
        TreeEvent::FullTreeReceived => {
            info!("Full tree received after {:?}", start.elapsed());
        }
    }

    Ok(())
}

async fn process_tree_element(parent: RelativeOid, node: TreeNode, wb: &Worterbuch) -> Result<()> {
    let oid = node.oid(&parent);

    #[cfg(feature = "tracing")]
    debug!("Got update for content of node {parent}: {node}");

    match node {
        TreeNode::Parameter(param) => {
            if let Some(value) = param.value() {
                publish(oid, value, wb).await?;
            }
        }
        TreeNode::QualifiedParameter(param) => {
            if let Some(value) = param.value() {
                publish(oid, value, wb).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn publish(oid: RelativeOid, value: Value, wb: &Worterbuch) -> Result<()> {
    let key = key(oid);
    wb.set(key, value).await?;
    Ok(())
}

fn key(oid: RelativeOid) -> String {
    format!("ember{}", oid.to_string().replace(".", "/"))
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
