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
    error::EmberResult,
    glow::Root,
    provider::{ClientHandler, start_tcp_provider},
};
use std::future::pending;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tracing")]
use tracing::{error, info, trace};

#[derive(Debug, Clone)]
struct EmberClientHandler {}

impl ClientHandler for EmberClientHandler {
    async fn handle_client(
        &self,
        tx: mpsc::Sender<Root>,
        mut rx: mpsc::Receiver<Root>,
    ) -> EmberResult<()> {
        while let Some(msg) = rx.recv().await {
            #[cfg(feature = "tracing")]
            trace!("Received ember message: {msg:?}");
            // TODO
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> EmberResult<()> {
    #[cfg(feature = "tracing")]
    logging::init();

    let local_addr = "0.0.0.0:9000".parse().expect("malformed socket address");
    let keepalive = None;
    let use_non_escaping = false;
    let client_handler = EmberClientHandler {};
    let cancellation_token = CancellationToken::new();

    start_tcp_provider(
        local_addr,
        keepalive,
        use_non_escaping,
        client_handler,
        cancellation_token,
    )
    .await?;

    pending::<()>().await;

    Ok(())
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
