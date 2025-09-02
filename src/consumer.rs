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
    s101::{EmberPacket, S101Frame},
};
use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    spawn,
    sync::mpsc,
};
use tracing::{error, info, warn};

pub async fn start_consumer(
    producer: SocketAddr,
) -> EmberResult<(mpsc::Sender<EmberPacket>, mpsc::Receiver<EmberPacket>)> {
    info!("Connecting to provider {producer} …");

    let socket = TcpStream::connect(producer).await?;
    socket.set_nodelay(true)?;

    info!("Successfully connected.");

    let (send_tx, send_rx) = mpsc::channel(1024);
    let (receive_tx, receive_rx) = mpsc::channel(1024);
    let (sock_rx, sock_tx) = socket.into_split();

    spawn(send(send_rx, sock_tx));
    spawn(receive(sock_rx, receive_tx));

    Ok((send_tx, receive_rx))
}

async fn send(mut rx: mpsc::Receiver<EmberPacket>, mut sock: OwnedWriteHalf) {
    let mut encode_buf = [0u8; 65536];
    let mut out_buf = Vec::new();

    // TODO socket timeouts
    while let Some(packet) = rx.recv().await {
        S101Frame::EmberPacket(packet).encode_escaping(&mut encode_buf, &mut out_buf);
        if let Err(e) = sock.write_all(&out_buf).await {
            error!("Could not write to TCP stream: {e}");
            break;
        }
        out_buf.clear();
    }
}

async fn receive(mut sock: OwnedReadHalf, tx: mpsc::Sender<EmberPacket>) {
    let mut buf = [0u8; 65536];

    loop {
        match S101Frame::decode(&mut sock, &mut buf).await {
            Ok(Some(S101Frame::EmberPacket(packet))) => {
                if tx.send(packet).await.is_err() {
                    break;
                }
            }
            Ok(Some(S101Frame::KeepaliveRequest)) => {
                // TODO
                info!("received keepalive request");
            }
            Ok(Some(S101Frame::KeepaliveResponse)) => {
                // TODO
                info!("received keepalive response");
            }
            Ok(None) => {}
            Err(e) => match e {
                EmberError::Deserialization(e) => {
                    warn!("Could not deserialize S101 frame: {e}");
                }
                EmberError::Io(e) => {
                    error!("Could not read from TCP stream: {e}");
                    break;
                }
            },
        }
    }

    // TODO
}
