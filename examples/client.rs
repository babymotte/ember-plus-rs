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
use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::main]
async fn main() -> EmberResult<()> {
    tracing_subscriber::fmt().init();

    let cancel = CancellationToken::new();

    let consumer = start_tcp_consumer(
        "127.0.0.1:9000".parse().expect("malformed socket address"),
        // Some(Duration::from_secs(1)),
        None,
        false,
        cancel.clone(),
    )
    .await?;

    let mut rx = consumer.fetch_full_tree().await;

    while let Some((parent, node)) = rx.recv().await {
        info!("Received node: {:?}: {:?}", parent, node);
    }

    cancel.cancelled().await;
    info!("Client closed.");

    Ok(())
}
