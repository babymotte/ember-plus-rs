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

use ember_plus_rs::{consumer::start_tcp_consumer, error::EmberResult};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::main]
async fn main() -> EmberResult<()> {
    logging::init();

    let shutdown_token = CancellationToken::new();

    let consumer = start_tcp_consumer(
        "127.0.0.1:9000".parse().expect("malformed socket address"),
        // Some(Duration::from_secs(1)),
        None,
        false,
        shutdown_token.clone(),
    )
    .await?;

    let mut rx = consumer.fetch_full_tree().await;

    loop {
        select! {
            Some((oid, node)) = rx.recv() => {
                info!("Received change in node {}: {:?}", oid, node);
            },
            _ = shutdown_token.cancelled() => break,
            else => break,
        }
    }

    info!("Client closed.");

    Ok(())
}

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
