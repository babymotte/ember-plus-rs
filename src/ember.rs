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
    glow::{GLOW_VERSION_MAJOR, GLOW_VERSION_MINOR, Root},
    s101::Flags,
};
use rasn::ber;

// TODO figure out correct max payload len
const MAX_PAYLOAD_LEN: usize = usize::MAX;

#[derive(Debug, Clone, PartialEq)]
pub struct EmberPacket {
    flag: Flags,
    dtd: u8,
    app_bytes: u8,
    glow_version_maj: u8,
    glow_version_min: u8,
    payload: Vec<u8>,
}

impl EmberPacket {
    pub fn new(flag: Flags, glow_version_maj: u8, glow_version_min: u8, payload: Vec<u8>) -> Self {
        Self {
            flag,
            dtd: 0x01,
            app_bytes: 0x02,
            glow_version_maj,
            glow_version_min,
            payload,
        }
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.flag = flag;
    }

    pub fn set_glow_dtd_version(&mut self, major: u8, minor: u8) {
        self.glow_version_maj = major;
        self.glow_version_min = minor;
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        &mut self.payload
    }

    pub fn len(&self) -> usize {
        self.payload.len() + 3 + self.app_bytes as usize
    }

    pub fn is_empty(&self) -> bool {
        self.flag == Flags::EmptyPacket
    }

    pub fn to_bytes(&self, buf: &mut [u8]) {
        if buf.len() < self.len() {
            panic!("insufficient buffer size")
        }

        buf[0] = self.flag as u8;
        buf[1] = self.dtd;
        buf[2] = self.app_bytes;
        buf[3] = self.glow_version_min;
        buf[4] = self.glow_version_maj;
        (&mut buf[5..]).copy_from_slice(&self.payload);
    }

    pub fn from_bytes(buf: &[u8]) -> EmberResult<Self> {
        if buf.len() <= 5 {
            return Err(EmberError::Deserialization(format!(
                "Invalid payload length {} (minimum is 6)",
                buf.len()
            )));
        }
        Ok(Self {
            flag: Flags::try_from(buf[0] as i32)?,
            dtd: buf[1],
            app_bytes: buf[2],
            glow_version_min: buf[3],
            glow_version_maj: buf[4],
            payload: buf[5..].to_vec(),
        })
    }
}

impl TryFrom<Root> for EmberPacket {
    type Error = EmberError;

    fn try_from(value: Root) -> Result<Self, Self::Error> {
        let payload = ber::encode(&value)?;
        if payload.len() > MAX_PAYLOAD_LEN {
            // TODO split into multiple frames
            todo!()
        } else {
            Ok(EmberPacket::new(
                Flags::SinglePacket,
                GLOW_VERSION_MAJOR,
                GLOW_VERSION_MINOR,
                payload,
            ))
        }
    }
}
