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

use miette::Diagnostic;
use rasn::error::{DecodeError, EncodeError};
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum EmberError {
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("S101 Decoder error")]
    S101DecodeError,
    #[error("BER encode error: {0}")]
    BerEncodeError(#[from] EncodeError),
    #[error("BER decode error: {0}")]
    BerDecodeError(#[from] DecodeError),
}

pub type EmberResult<T> = Result<T, EmberError>;
