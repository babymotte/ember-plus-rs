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
    ember::EmberPacket,
    error::EmberError,
    glow::Root,
    s101::{EscapingS101Frame, Flags, NonEscapingS101Frame, S101Frame},
    utils::format_bytes,
};
use std::time::Duration;
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
#[cfg(feature = "tracing")]
use tracing::{debug, error, trace, warn};

const ENCODE_BUFFER_SIZE: usize = 1290;

pub async fn ember_client_channel(
    keepalive: Option<Duration>,
    socket: TcpStream,
    try_use_non_escaping: bool,
) -> Result<(mpsc::Sender<Root>, mpsc::Receiver<Root>), EmberError> {
    ember_channel(keepalive, socket, try_use_non_escaping, true).await
}

pub async fn ember_server_channel(
    keepalive: Option<Duration>,
    socket: TcpStream,
    use_non_escaping: bool,
) -> Result<(mpsc::Sender<Root>, mpsc::Receiver<Root>), EmberError> {
    ember_channel(keepalive, socket, use_non_escaping, false).await
}

async fn ember_channel(
    keepalive: Option<Duration>,
    mut socket: TcpStream,
    try_use_non_escaping: bool,
    negotiate: bool,
) -> Result<(mpsc::Sender<Root>, mpsc::Receiver<Root>), EmberError> {
    let mut encode_buf = [0u8; ENCODE_BUFFER_SIZE];
    let out_buf = Vec::new();

    let channel_buf_size = 1024 * 1024;

    let (packetize_tx, packetize_rx) = mpsc::channel(channel_buf_size);
    let (frame_tx, frame_rx) = mpsc::channel(channel_buf_size);
    let (send_tx, send_rx) = mpsc::channel(channel_buf_size);
    let (receive_tx, receive_rx) = mpsc::channel(channel_buf_size);
    let (unframe_tx, unframe_rx) = mpsc::channel(channel_buf_size);
    let (depacketize_tx, depacketize_rx) = mpsc::channel(channel_buf_size);

    let use_non_escaping = if negotiate {
        try_use_non_escaping
            && negotiate_non_escaping(&mut socket, &mut encode_buf, &receive_tx).await?
    } else {
        try_use_non_escaping
    };

    let (sock_rx, sock_tx) = socket.into_split();
    let (keepalive_tx, keepalive_request_rx) = mpsc::channel(channel_buf_size);

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

    spawn(packetize(packetize_rx, frame_tx));
    spawn(frame(frame_rx, send_tx, use_non_escaping));
    spawn(send(send_rx, sock_tx, encode_buf, out_buf));
    spawn(receive(sock_rx, receive_tx, keepalive_tx));
    spawn(unframe(receive_rx, unframe_tx));
    spawn(depacketize(unframe_rx, depacketize_tx));

    Ok((packetize_tx, depacketize_rx))
}

async fn send_keepalive(
    keepalive: Duration,
    tx: mpsc::Sender<S101Frame>,
    mut keepalive_request_rx: mpsc::Receiver<()>,
    use_non_escaping: bool,
) {
    let mut interval = interval(keepalive);

    #[cfg(feature = "tracing")]
    debug!(
        "Starting keepalive loop, sending keepalive requests and responding to keepalive requests."
    );

    loop {
        select! {
                    _ = interval.tick() => {
                        let frame = if use_non_escaping {
        #[cfg(feature = "tracing")]
                            debug!("Sending non-escaping keepalive request");
                            S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveRequest)
                        } else {
        #[cfg(feature = "tracing")]
                            debug!("Sending escaping keepalive request");
                            S101Frame::Escaping(EscapingS101Frame::KeepaliveRequest)
                        };
                        if tx.send(frame).await.is_err() {
                            break;
                        }
                    }
                    _ = keepalive_request_rx.recv() => {
                        let frame = if use_non_escaping {
        #[cfg(feature = "tracing")]
                            debug!("Received keepalive request. Sending non-escaping keepalive response");
                            S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveResponse)
                        } else {
        #[cfg(feature = "tracing")]
                            debug!("Received keepalive request. Sending escaping keepalive response");
                            S101Frame::Escaping(EscapingS101Frame::KeepaliveResponse)
                        };
                        if tx.send(frame).await.is_err() {
                            break;
                        }
                    }
                }
    }

    #[cfg(feature = "tracing")]
    debug!("Keepalive loop stopped.");
}

async fn send_keepalive_response(
    tx: mpsc::Sender<S101Frame>,
    mut keepalive_request_rx: mpsc::Receiver<()>,
    use_non_escaping: bool,
) {
    #[cfg(feature = "tracing")]
    debug!("Starting keepalive loop, responding to keepalive requests.");

    loop {
        select! {
                    _ = keepalive_request_rx.recv() => {
                        let frame = if use_non_escaping {
        #[cfg(feature = "tracing")]
                            debug!("Received keepalive request. Sending non-escaping keepalive response");
                            S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveResponse)
                        } else {
        #[cfg(feature = "tracing")]
                            debug!("Received keepalive request. Sending escaping keepalive response");
                            S101Frame::Escaping(EscapingS101Frame::KeepaliveResponse)
                        };
                        if tx.send(frame).await.is_err() {
                            break;
                        }
                    }
                }
    }

    #[cfg(feature = "tracing")]
    debug!("Keepalive loop stopped.");
}

async fn send(
    mut rx: mpsc::Receiver<S101Frame>,
    mut sock: OwnedWriteHalf,
    mut encode_buf: [u8; ENCODE_BUFFER_SIZE],
    mut out_buf: Vec<u8>,
) {
    #[cfg(feature = "tracing")]
    debug!("Starting send loop.");

    // TODO socket timeouts
    while let Some(frame) = rx.recv().await {
        #[cfg(feature = "tracing")]
        trace!("Sending frame {frame:?} …");
        let send_buf = frame.encode(&mut encode_buf, &mut out_buf);
        if let Err(e) = sock.write_all(send_buf).await {
            #[cfg(feature = "tracing")]
            error!("Could not write to TCP stream: {e}");
            break;
        }
        out_buf.clear();
        #[cfg(feature = "tracing")]
        trace!("Sending frame done.");
    }

    #[cfg(feature = "tracing")]
    debug!("Send loop stopped.");
}

async fn receive(
    mut sock: OwnedReadHalf,
    tx: mpsc::Sender<S101Frame>,
    keepalive_tx: mpsc::Sender<()>,
) {
    let mut buf = [0u8; 65536];

    #[cfg(feature = "tracing")]
    debug!("Starting receive loop.");

    loop {
        match S101Frame::decode(&mut sock, &mut buf).await {
            Ok(Some(frame)) => {
                #[cfg(feature = "tracing")]
                trace!("Received frame: {frame:?}");
                if frame.is_keepalive_request() {
                    if keepalive_tx.send(()).await.is_err() {
                        break;
                    }
                } else if frame.is_keepalive_response() {
                    #[cfg(feature = "tracing")]
                    trace!("Received frame: {frame:?}");
                // TODO check for missing keepalive responses
                } else if tx.send(frame).await.is_err() {
                    break;
                }
            }
            Ok(None) => {}
            Err(e) => match e {
                EmberError::Deserialization(e) => {
                    #[cfg(feature = "tracing")]
                    warn!("Could not deserialize S101 frame: {e}");
                }
                e => {
                    #[cfg(feature = "tracing")]
                    error!("Error receiving next frame: {e}");
                    break;
                }
            },
        }
    }

    #[cfg(feature = "tracing")]
    debug!("Receive loop stopped.");
}

async fn packetize(mut rx: mpsc::Receiver<Root>, tx: mpsc::Sender<EmberPacket>) {
    #[cfg(feature = "tracing")]
    debug!("Starting packetize loop.");

    while let Some(msg) = rx.recv().await {
        let packets = match msg.to_packets() {
            Ok(it) => it,
            Err(e) => {
                #[cfg(feature = "tracing")]
                error!("Error packetizing ember message: {e}");
                continue;
            }
        };

        for packet in packets {
            if tx.send(packet).await.is_err() {
                break;
            }
        }
    }

    #[cfg(feature = "tracing")]
    debug!("Packetize loop stopped.");
}

async fn frame(
    mut rx: mpsc::Receiver<EmberPacket>,
    tx: mpsc::Sender<S101Frame>,
    use_non_escaping: bool,
) {
    #[cfg(feature = "tracing")]
    debug!("Starting frame loop.");

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

    #[cfg(feature = "tracing")]
    debug!("Frame loop stopped.");
}

async fn unframe(mut rx: mpsc::Receiver<S101Frame>, tx: mpsc::Sender<EmberPacket>) {
    #[cfg(feature = "tracing")]
    debug!("Starting unframe loop.");

    while let Some(frame) = rx.recv().await {
        match frame {
            S101Frame::Escaping(EscapingS101Frame::EmberPacket(packet))
            | S101Frame::NonEscaping(NonEscapingS101Frame::EmberPacket(packet)) => {
                #[cfg(feature = "tracing")]
                trace!("Received EmBER+ packet: {packet:?}");
                if tx.send(packet).await.is_err() {
                    break;
                }
            }
            _ => {}
        }
    }

    #[cfg(feature = "tracing")]
    debug!("Unframe loop stopped.");
}

async fn depacketize(mut rx: mpsc::Receiver<EmberPacket>, tx: mpsc::Sender<Root>) {
    #[cfg(feature = "tracing")]
    debug!("Starting de-packetize loop.");

    let mut buf = Vec::new();

    while let Some(packet) = rx.recv().await {
        let root = match packet.flag() {
            Flags::SinglePacket => {
                if !buf.is_empty() {
                    #[cfg(feature = "tracing")]
                    warn!(
                        "Received single packet EmBER+ message while re-constructing multi-packet message. Discarding partial message."
                    );
                    buf.clear();
                }
                #[cfg(feature = "tracing")]
                trace!("Received single packet EmBER+ message.");
                let packets = [packet];
                let root = match Root::from_packets(&packets) {
                    Ok(it) => it,
                    Err(e) => {
                        #[cfg(feature = "tracing")]
                        error!(
                            "Error de-packetizing ember message from packet:\n{}\n: {}",
                            format_bytes(packets[0].payload()),
                            e
                        );
                        continue;
                    }
                };
                #[cfg(feature = "tracing")]
                trace!("Decoded single packet EmBER+ message: {root:?}");
                Some(root)
            }
            Flags::MultiPacketFirst => {
                if !buf.is_empty() {
                    #[cfg(feature = "tracing")]
                    warn!(
                        "Received start of multi-packet EmBER+ message while re-constructing another multi-packet message. Discarding partial message."
                    );
                    buf.clear();
                }
                #[cfg(feature = "tracing")]
                trace!("Received start of multi-packet EmBER+ message …");
                buf.push(packet);
                None
            }
            Flags::MultiPacket => {
                if buf.is_empty() {
                    #[cfg(feature = "tracing")]
                    warn!(
                        "Received intermediate part of multi-packet EmBER+ message but there was no previous packet in this message. Discarding partial message."
                    );
                    continue;
                }
                #[cfg(feature = "tracing")]
                trace!("Received intermediate part of multi-packet EmBER+ message …");
                buf.push(packet);
                None
            }
            Flags::MultiPacketLast => {
                if buf.is_empty() {
                    #[cfg(feature = "tracing")]
                    warn!(
                        "Received end of multi-packet EmBER+ message but there was no previous packet in this message. Discarding partial message."
                    );
                    continue;
                }
                buf.push(packet);
                let packets = buf.clone();
                buf.clear();
                let num_parts = packets.len();
                if num_parts >= 500 {
                    #[cfg(feature = "tracing")]
                    warn!("Received end of multi-packet EmBER+ message with {num_parts} parts.");
                } else {
                    #[cfg(feature = "tracing")]
                    trace!("Received end of multi-packet EmBER+ message with {num_parts} parts.");
                }
                let root = match Root::from_packets(&packets) {
                    Ok(it) => it,
                    Err(e) => {
                        #[cfg(feature = "tracing")]
                        error!(
                            "Error de-packetizing ember message from packets:\n{}\n: {}",
                            packets
                                .iter()
                                .map(|it| format_bytes(it.payload()))
                                .collect::<Vec<String>>()
                                .join("\n"),
                            e
                        );
                        continue;
                    }
                };
                #[cfg(feature = "tracing")]
                trace!("Decoded multi-packet EmBER+ message: {root:?}");
                Some(root)
            }
            Flags::EmptyPacket => None,
        };

        if let Some(root) = root {
            if tx.send(root).await.is_err() {
                break;
            }
        };
    }

    #[cfg(feature = "tracing")]
    debug!("De-packetize loop stopped.");
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
            #[cfg(feature = "tracing")]
            debug!("Did not receive a response, falling back to escaping mode.");
            break Ok(false);
        }

        #[cfg(feature = "tracing")]
        debug!(
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
                    #[cfg(feature = "tracing")]
                    debug!("Received non-escaping response, switching to non-escaping mode.");
                } else {
                    #[cfg(feature = "tracing")]
                    debug!("Received escaping response, falling back to escaping mode.");
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
