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
    error::{EmberError, EmberResult},
    s101::{EmberPacket, EscapingS101Frame, NonEscapingS101Frame, S101Frame},
};
use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    select, spawn,
    sync::mpsc,
    time::{interval, timeout},
};
use tracing::{error, info, trace, warn};

const ENCODE_BUFFER_SIZE: usize = 65536;

pub async fn start_tcp_consumer(
    producer: SocketAddr,
    keepalive: Option<Duration>,
    try_use_non_escaping: bool,
) -> EmberResult<(mpsc::Sender<EmberPacket>, mpsc::Receiver<EmberPacket>)> {
    info!("Connecting to provider {producer} …");

    let mut socket = TcpStream::connect(producer).await?;
    socket.set_nodelay(true)?;

    info!("Successfully connected.");

    let mut encode_buf = [0u8; 65536];
    let out_buf = Vec::new();

    let (send_tx, send_rx) = mpsc::channel(1024);
    let (receive_tx, receive_rx) = mpsc::channel(1024);

    let use_non_escaping = try_use_non_escaping
        && negotiate_non_escaping(&mut socket, &mut encode_buf, &receive_tx).await?;

    let (sock_rx, sock_tx) = socket.into_split();
    let (keepalive_tx, keepalive_request_rx) = mpsc::channel(1);

    if let Some(keepalive) = keepalive {
        spawn(send_keepalive(
            keepalive,
            send_tx.clone(),
            keepalive_request_rx,
            use_non_escaping,
        ));
    } else {
        spawn(send_keepalive_response(
            send_tx.clone(),
            keepalive_request_rx,
            use_non_escaping,
        ));
    }
    spawn(send(send_rx, sock_tx, encode_buf, out_buf));
    spawn(receive(sock_rx, receive_tx, keepalive_tx));

    let (wrapper_tx, wrapper_rx) = mpsc::channel(1);
    let (unwrapper_tx, unwrapper_rx) = mpsc::channel(1);

    spawn(wrap(wrapper_rx, send_tx, use_non_escaping));
    spawn(unwrap(receive_rx, unwrapper_tx));

    Ok((wrapper_tx, unwrapper_rx))
}

async fn negotiate_non_escaping(
    mut socket: &mut TcpStream,
    encode_buf: &mut [u8],
    receive_tx: &mpsc::Sender<S101Frame>,
) -> Result<bool, EmberError> {
    let mut attempt = 0;
    let max = 3;
    loop {
        if attempt >= max {
            info!("Did not receive a response, falling back to escaping mode.");
            break Ok(false);
        }

        info!(
            "Sending initial non-escaping keepalive request ({}/{}) …",
            attempt + 1,
            max
        );
        let frame = NonEscapingS101Frame::KeepaliveRequest;
        frame.encode(encode_buf);
        let send_buf = &encode_buf[..frame.encoded_len()];
        socket.write_all(send_buf).await?;

        let decode = S101Frame::decode(&mut socket, encode_buf);

        match timeout(Duration::from_secs(1), decode).await {
            Ok(Ok(Some(frame))) => {
                let non_escaping = frame.is_non_escaping();
                if non_escaping {
                    info!("Received non-escaping response, switching to non-escaping mode.");
                } else {
                    info!("Received escaping response, falling back to escaping mode.");
                }
                if receive_tx.send(frame).await.is_err() {
                    return Err(EmberError::Connection("Connection closed.".to_owned()));
                }
                return Ok(non_escaping);
            }
            Ok(Err(e)) => return Err(e),
            _ => attempt += 1,
        }
    }
}

async fn send_keepalive(
    keepalive: Duration,
    tx: mpsc::Sender<S101Frame>,
    mut keepalive_request_rx: mpsc::Receiver<()>,
    use_non_escaping: bool,
) {
    let mut interval = interval(keepalive);

    info!(
        "Starting keepalive loop, sending keepalive requests and responding to keepalive requests."
    );

    loop {
        select! {
            _ = interval.tick() => {
                let frame = if use_non_escaping {
                    trace!("Sending non-escaping keepalive request");
                    S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveRequest)
                } else {
                    trace!("Sending escaping keepalive request");
                    S101Frame::Escaping(EscapingS101Frame::KeepaliveRequest)
                };
                if tx.send(frame).await.is_err() {
                    break;
                }
            }
            _ = keepalive_request_rx.recv() => {
                let frame = if use_non_escaping {
                    trace!("Received keepalive request. Sending non-escaping keepalive response");
                    S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveResponse)
                } else {
                    trace!("Received keepalive request. Sending escaping keepalive response");
                    S101Frame::Escaping(EscapingS101Frame::KeepaliveResponse)
                };
                if tx.send(frame).await.is_err() {
                    break;
                }
            }
        }
    }

    info!("Keepalive loop stopped.");
}

async fn send_keepalive_response(
    tx: mpsc::Sender<S101Frame>,
    mut keepalive_request_rx: mpsc::Receiver<()>,
    use_non_escaping: bool,
) {
    info!("Starting keepalive loop, responding to keepalive requests.");

    loop {
        select! {
            _ = keepalive_request_rx.recv() => {
                let frame = if use_non_escaping {
                    trace!("Received keepalive request. Sending non-escaping keepalive response");
                    S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveResponse)
                } else {
                    trace!("Received keepalive request. Sending escaping keepalive response");
                    S101Frame::Escaping(EscapingS101Frame::KeepaliveResponse)
                };
                if tx.send(frame).await.is_err() {
                    break;
                }
            }
        }
    }

    info!("Keepalive loop stopped.");
}

async fn send(
    mut rx: mpsc::Receiver<S101Frame>,
    mut sock: OwnedWriteHalf,
    mut encode_buf: [u8; ENCODE_BUFFER_SIZE],
    mut out_buf: Vec<u8>,
) {
    info!("Starting receive loop.");

    // TODO socket timeouts
    while let Some(frame) = rx.recv().await {
        trace!("Sending frame {frame:?} …");
        let send_buf = frame.encode(&mut encode_buf, &mut out_buf);
        if let Err(e) = sock.write_all(send_buf).await {
            error!("Could not write to TCP stream: {e}");
            break;
        }
        out_buf.clear();
        trace!("Sending frame done.");
    }

    info!("Send loop stopped.");
}

async fn receive(
    mut sock: OwnedReadHalf,
    tx: mpsc::Sender<S101Frame>,
    keepalive_tx: mpsc::Sender<()>,
) {
    let mut buf = [0u8; 65536];

    info!("Starting receive loop.");

    loop {
        match S101Frame::decode(&mut sock, &mut buf).await {
            Ok(Some(frame)) => {
                trace!("Received frame: {frame:?}");
                if frame.is_keepalive_request() {
                    if keepalive_tx.send(()).await.is_err() {
                        break;
                    }
                }
                if tx.send(frame).await.is_err() {
                    break;
                }
            }
            Ok(None) => {}
            Err(e) => match e {
                EmberError::Deserialization(e) => {
                    warn!("Could not deserialize S101 frame: {e}");
                }
                e => {
                    error!("Error receiving next frame: {e}");
                    break;
                }
            },
        }
    }

    info!("Receive loop stopped.");
}

async fn wrap(
    mut rx: mpsc::Receiver<EmberPacket>,
    tx: mpsc::Sender<S101Frame>,
    use_non_escaping: bool,
) {
    info!("Starting wrap loop.");

    while let Some(packet) = rx.recv().await {
        let frame = if use_non_escaping {
            S101Frame::NonEscaping(NonEscapingS101Frame::EmberPacket(packet))
        } else {
            S101Frame::Escaping(EscapingS101Frame::EmberPacket(packet))
        };
        if tx.send(frame).await.is_err() {
            break;
        }
    }

    info!("Wrap loop stopped.");
}

async fn unwrap(mut rx: mpsc::Receiver<S101Frame>, tx: mpsc::Sender<EmberPacket>) {
    info!("Starting unwrap loop.");

    while let Some(frame) = rx.recv().await {
        match frame {
            S101Frame::Escaping(EscapingS101Frame::EmberPacket(packet))
            | S101Frame::NonEscaping(NonEscapingS101Frame::EmberPacket(packet)) => {
                if tx.send(packet).await.is_err() {
                    break;
                }
            }
            _ => {}
        }
    }

    info!("Unwrap loop stopped.");
}
