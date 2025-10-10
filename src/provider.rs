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

use crate::{com::ember_server_channel, error::EmberResult, glow::Root};
use std::{io, net::SocketAddr, time::Duration};
use tokio::{net::TcpListener, select, spawn, sync::mpsc};
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tracing")]
use tracing::{error, info};

pub trait ClientHandler: Clone + Send + Sync + 'static {
    fn handle_client(
        &self,
        tx: mpsc::Sender<Root>,
        rx: mpsc::Receiver<Root>,
    ) -> impl Future<Output = EmberResult<()>> + Send;
}

pub async fn start_tcp_provider(
    local_addr: SocketAddr,
    keepalive: Option<Duration>,
    use_non_escaping: bool,
    client_handler: impl ClientHandler,
    cancellation_token: CancellationToken,
) -> EmberResult<()> {
    #[cfg(feature = "tracing")]
    info!("Starting provider at {local_addr} …");

    // TODO set up socket correctly
    let socket = TcpListener::bind(local_addr).await?;

    spawn(accept_clients(
        keepalive,
        use_non_escaping,
        client_handler,
        cancellation_token,
        socket,
    ));

    Ok(())
}

async fn accept_clients(
    keepalive: Option<Duration>,
    use_non_escaping: bool,
    client_handler: impl ClientHandler,
    cancellation_token: CancellationToken,
    socket: TcpListener,
) {
    loop {
        select! {
            client = socket.accept() => {
                let client_handler = client_handler.clone();
                if !client_accepted(client, keepalive, use_non_escaping, client_handler).await {
                break;
            }
            },
            _ = cancellation_token.cancelled() => {
                #[cfg(feature = "tracing")]
                info!("Received stop signal.");
                break;
            },
        }
    }
}

async fn client_accepted(
    client: io::Result<(tokio::net::TcpStream, SocketAddr)>,
    keepalive: Option<Duration>,
    use_non_escaping: bool,
    client_handler: impl ClientHandler,
) -> bool {
    match client {
        Ok((client, addr)) => {
            client_connected(keepalive, use_non_escaping, client_handler, client, addr).await;
        }
        Err(e) => {
            #[cfg(feature = "tracing")]
            error!("Erro accpting client connection: {e}");
            return false;
        }
    }
    true
}

async fn client_connected(
    keepalive: Option<Duration>,
    use_non_escaping: bool,
    client_handler: impl ClientHandler,
    client: tokio::net::TcpStream,
    addr: SocketAddr,
) {
    #[cfg(feature = "tracing")]
    info!("New client connected: {addr}");
    match ember_server_channel(keepalive, client, use_non_escaping).await {
        Ok((ember_tx, ember_rx)) => {
            serve(client_handler, addr, ember_tx, ember_rx).await;
        }
        Err(e) => {
            #[cfg(feature = "tracing")]
            error!("Error establishing ember+ communication with client {addr}: {e}");
        }
    }
}

async fn serve(
    client_handler: impl ClientHandler,
    addr: SocketAddr,
    ember_tx: mpsc::Sender<Root>,
    ember_rx: mpsc::Receiver<Root>,
) {
    spawn(async move {
        if let Err(e) = client_handler.handle_client(ember_tx, ember_rx).await {
            #[cfg(feature = "tracing")]
            error!("Client connection {addr} closed unexpectedly: {e}");
        }
    });
}
