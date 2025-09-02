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

use ember_plus_rs::{consumer::start_consumer, error::EmberResult};
use tracing::info;

#[tokio::main]
async fn main() -> EmberResult<()> {
    tracing_subscriber::fmt().init();

    let (tx, mut rx) =
        start_consumer("127.0.0.1:9000".parse().expect("malformed socket address")).await?;

    while let Some(packet) = rx.recv().await {
        info!("Received {packet:?}");
    }

    Ok(())
}
