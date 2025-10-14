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
    back_to_enum,
    ember::EmberPacket,
    error::{EmberError, EmberResult},
};
use serde::{Deserialize, Serialize};
use std::{fmt, io::Read, slice};
use tokio::io::{AsyncRead, AsyncReadExt};

pub const BOF: u8 = 0xFE;
pub const EOF: u8 = 0xFF;
pub const CE: u8 = 0xFD;
pub const XOR: u8 = 0x20;
pub const BOFNE: u8 = 0xF8;
pub const CRC_SEED: u16 = 0xFFFF;
pub const CRC_CHECK: u16 = 0xF0B8;
pub const SLOT_IDENTIFIER: u8 = 0x00;
pub const MESSAGE_TYPE: u8 = 0x0E;
pub const COMMAND_EMBER_PACKET: u8 = 0x00;
pub const COMMAND_KEEPALIVE_REQUEST: u8 = 0x01;
pub const COMMAND_KEEPALIVE_RESPONSE: u8 = 0x02;
pub const VERSION: u8 = 0x01;
pub const FLAG_SINGLE_PACKET: u8 = 0xC0;
pub const FLAG_MULTI_PACKET_FIRST: u8 = 0x80;
pub const FLAG_MULTI_PACKET_LAST: u8 = 0x40;
pub const FLAG_EMPTY_PACKET: u8 = 0x20;
pub const FLAG_MULTI_PACKET: u8 = 0x00;
pub const CRC_TABLE: &[u16] = &[
    0x0000, 0x1189, 0x2312, 0x329b, 0x4624, 0x57ad, 0x6536, 0x74bf, 0x8c48, 0x9dc1, 0xaf5a, 0xbed3,
    0xca6c, 0xdbe5, 0xe97e, 0xf8f7, 0x1081, 0x0108, 0x3393, 0x221a, 0x56a5, 0x472c, 0x75b7, 0x643e,
    0x9cc9, 0x8d40, 0xbfdb, 0xae52, 0xdaed, 0xcb64, 0xf9ff, 0xe876, 0x2102, 0x308b, 0x0210, 0x1399,
    0x6726, 0x76af, 0x4434, 0x55bd, 0xad4a, 0xbcc3, 0x8e58, 0x9fd1, 0xeb6e, 0xfae7, 0xc87c, 0xd9f5,
    0x3183, 0x200a, 0x1291, 0x0318, 0x77a7, 0x662e, 0x54b5, 0x453c, 0xbdcb, 0xac42, 0x9ed9, 0x8f50,
    0xfbef, 0xea66, 0xd8fd, 0xc974, 0x4204, 0x538d, 0x6116, 0x709f, 0x0420, 0x15a9, 0x2732, 0x36bb,
    0xce4c, 0xdfc5, 0xed5e, 0xfcd7, 0x8868, 0x99e1, 0xab7a, 0xbaf3, 0x5285, 0x430c, 0x7197, 0x601e,
    0x14a1, 0x0528, 0x37b3, 0x263a, 0xdecd, 0xcf44, 0xfddf, 0xec56, 0x98e9, 0x8960, 0xbbfb, 0xaa72,
    0x6306, 0x728f, 0x4014, 0x519d, 0x2522, 0x34ab, 0x0630, 0x17b9, 0xef4e, 0xfec7, 0xcc5c, 0xddd5,
    0xa96a, 0xb8e3, 0x8a78, 0x9bf1, 0x7387, 0x620e, 0x5095, 0x411c, 0x35a3, 0x242a, 0x16b1, 0x0738,
    0xffcf, 0xee46, 0xdcdd, 0xcd54, 0xb9eb, 0xa862, 0x9af9, 0x8b70, 0x8408, 0x9581, 0xa71a, 0xb693,
    0xc22c, 0xd3a5, 0xe13e, 0xf0b7, 0x0840, 0x19c9, 0x2b52, 0x3adb, 0x4e64, 0x5fed, 0x6d76, 0x7cff,
    0x9489, 0x8500, 0xb79b, 0xa612, 0xd2ad, 0xc324, 0xf1bf, 0xe036, 0x18c1, 0x0948, 0x3bd3, 0x2a5a,
    0x5ee5, 0x4f6c, 0x7df7, 0x6c7e, 0xa50a, 0xb483, 0x8618, 0x9791, 0xe32e, 0xf2a7, 0xc03c, 0xd1b5,
    0x2942, 0x38cb, 0x0a50, 0x1bd9, 0x6f66, 0x7eef, 0x4c74, 0x5dfd, 0xb58b, 0xa402, 0x9699, 0x8710,
    0xf3af, 0xe226, 0xd0bd, 0xc134, 0x39c3, 0x284a, 0x1ad1, 0x0b58, 0x7fe7, 0x6e6e, 0x5cf5, 0x4d7c,
    0xc60c, 0xd785, 0xe51e, 0xf497, 0x8028, 0x91a1, 0xa33a, 0xb2b3, 0x4a44, 0x5bcd, 0x6956, 0x78df,
    0x0c60, 0x1de9, 0x2f72, 0x3efb, 0xd68d, 0xc704, 0xf59f, 0xe416, 0x90a9, 0x8120, 0xb3bb, 0xa232,
    0x5ac5, 0x4b4c, 0x79d7, 0x685e, 0x1ce1, 0x0d68, 0x3ff3, 0x2e7a, 0xe70e, 0xf687, 0xc41c, 0xd595,
    0xa12a, 0xb0a3, 0x8238, 0x93b1, 0x6b46, 0x7acf, 0x4854, 0x59dd, 0x2d62, 0x3ceb, 0x0e70, 0x1ff9,
    0xf78f, 0xe606, 0xd49d, 0xc514, 0xb1ab, 0xa022, 0x92b9, 0x8330, 0x7bc7, 0x6a4e, 0x58d5, 0x495c,
    0x3de3, 0x2c6a, 0x1ef1, 0x0f78,
];

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Flags {
    SinglePacket = FLAG_SINGLE_PACKET as isize,
    MultiPacketFirst = FLAG_MULTI_PACKET_FIRST as isize,
    MultiPacket = FLAG_MULTI_PACKET as isize,
    MultiPacketLast = FLAG_MULTI_PACKET_LAST as isize,
    EmptyPacket = FLAG_EMPTY_PACKET as isize,
}}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum S101Frame {
    Escaping(EscapingS101Frame),
    NonEscaping(NonEscapingS101Frame),
}

impl S101Frame {
    pub fn decode_blocking(mut data: impl Read, buf: &mut [u8]) -> EmberResult<Option<S101Frame>> {
        data.read_exact(&mut buf[..1])?;

        if buf[0] == BOF {
            EscapingS101Frame::decode_blocking(data, buf)
                .map(Self::Escaping)
                .map(Some)
        } else if buf[0] == BOFNE {
            NonEscapingS101Frame::decode_blocking(data, buf).map(|it| it.map(Self::NonEscaping))
        } else {
            Err(EmberError::Deserialization(format!(
                "invalid first byte: {:#04x}",
                buf[0]
            )))
        }
    }

    pub async fn decode<R: AsyncRead + Unpin>(
        mut data: R,
        buf: &mut [u8],
    ) -> EmberResult<Option<S101Frame>> {
        data.read_exact(&mut buf[..1]).await?;

        if buf[0] == BOF {
            EscapingS101Frame::decode(data, buf)
                .await
                .map(Self::Escaping)
                .map(Some)
        } else if buf[0] == BOFNE {
            NonEscapingS101Frame::decode(data, buf)
                .await
                .map(|it| it.map(Self::NonEscaping))
        } else {
            Err(EmberError::Deserialization(format!(
                "invalid first byte: {:#04x}",
                buf[0]
            )))
        }
    }

    pub(crate) fn encode<'a>(
        &self,
        encode_buf: &'a mut [u8],
        out_buf: &'a mut Vec<u8>,
    ) -> &'a [u8] {
        match self {
            S101Frame::Escaping(frame) => {
                frame.encode(encode_buf, out_buf);
                out_buf
            }
            S101Frame::NonEscaping(frame) => {
                frame.encode(encode_buf);
                &encode_buf[..frame.encoded_len()]
            }
        }
    }

    pub(crate) fn is_non_escaping(&self) -> bool {
        match self {
            S101Frame::Escaping(_) => false,
            S101Frame::NonEscaping(_) => true,
        }
    }

    pub(crate) fn is_keepalive_request(&self) -> bool {
        match self {
            S101Frame::Escaping(EscapingS101Frame::KeepaliveRequest)
            | S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveRequest) => true,
            _ => false,
        }
    }

    pub(crate) fn is_keepalive_response(&self) -> bool {
        match self {
            S101Frame::Escaping(EscapingS101Frame::KeepaliveResponse)
            | S101Frame::NonEscaping(NonEscapingS101Frame::KeepaliveResponse) => true,
            _ => false,
        }
    }
}

impl fmt::Display for S101Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("invalid json")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EscapingS101Frame {
    EmberPacket(EmberPacket),
    KeepaliveRequest,
    KeepaliveResponse,
}

impl EscapingS101Frame {
    pub fn len(&self) -> usize {
        4 + match self {
            Self::EmberPacket(ember_packet) => ember_packet.len(),
            Self::KeepaliveRequest | Self::KeepaliveResponse => 0,
        }
    }

    pub fn encode(&self, encode_buf: &mut [u8], out_buf: &mut Vec<u8>) {
        self.to_bytes(encode_buf);
        let mut crc = CRC_SEED;
        out_buf.push(BOF);
        for b in &encode_buf[..self.len()] {
            crc = Self::update_crc(crc, *b);
            Self::append_escaping(out_buf, *b);
        }
        for b in (!crc).to_le_bytes() {
            Self::append_escaping(out_buf, b);
        }
        out_buf.push(EOF);
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        if buf.len() < self.len() {
            panic!("insufficient buffer size")
        }

        buf[0] = SLOT_IDENTIFIER;
        buf[1] = MESSAGE_TYPE;
        buf[2] = self.command_byte();
        buf[3] = VERSION;

        if let Self::EmberPacket(data) = &self {
            data.to_bytes(&mut buf[4..self.len()]);
        }
    }

    fn command_byte(&self) -> u8 {
        match self {
            Self::EmberPacket(_) => COMMAND_EMBER_PACKET,
            Self::KeepaliveRequest => COMMAND_KEEPALIVE_REQUEST,
            Self::KeepaliveResponse => COMMAND_KEEPALIVE_RESPONSE,
        }
    }

    fn decode_blocking(mut data: impl Read, buf: &mut [u8]) -> EmberResult<Self> {
        let mut crc = CRC_SEED;
        let mut xor = false;
        let mut b: u8 = 0x00;
        let mut pos = 0;
        loop {
            data.read_exact(slice::from_mut(&mut b))?;
            if b == BOF {
                return Err(EmberError::Deserialization("Unexpected BOF".to_owned()));
            }

            if b == EOF {
                break;
            }

            if b == CE {
                xor = true;
                continue;
            }

            if xor {
                xor = false;
                b = b ^ XOR;
            }

            crc = Self::update_crc(crc, b);
            buf[pos] = b;
            pos += 1;
        }
        if crc != CRC_CHECK {
            return Err(EmberError::Deserialization(format!(
                "Invalid CRC: {crc:#06x}"
            )));
        }

        Self::from_bytes(&buf[..pos])
    }

    async fn decode<R: AsyncRead + Unpin>(mut data: R, buf: &mut [u8]) -> EmberResult<Self> {
        let mut crc = CRC_SEED;
        let mut xor = false;
        let mut b: u8 = 0x00;
        let mut pos = 0;
        loop {
            data.read_exact(slice::from_mut(&mut b)).await?;
            if b == BOF {
                return Err(EmberError::Deserialization("Unexpected BOF".to_owned()));
            }

            if b == EOF {
                break;
            }

            if b == CE {
                xor = true;
                continue;
            }

            if xor {
                xor = false;
                b = b ^ XOR;
            }

            crc = EscapingS101Frame::update_crc(crc, b);
            buf[pos] = b;
            pos += 1;
        }
        if crc != CRC_CHECK {
            return Err(EmberError::Deserialization(format!(
                "Invalid CRC: {crc:#06x}"
            )));
        }

        Self::from_bytes(&buf[..pos])
    }

    fn from_bytes(buf: &[u8]) -> EmberResult<EscapingS101Frame> {
        match buf[2] {
            COMMAND_EMBER_PACKET => {
                // the last two bytes are CRC
                EmberPacket::from_bytes(&buf[4..buf.len() - 2]).map(EscapingS101Frame::EmberPacket)
            }
            COMMAND_KEEPALIVE_REQUEST => Ok(EscapingS101Frame::KeepaliveRequest),
            COMMAND_KEEPALIVE_RESPONSE => Ok(EscapingS101Frame::KeepaliveRequest),
            it => Err(EmberError::Deserialization(format!(
                "Invalid command byte: {:#04x}",
                it
            ))),
        }
    }

    fn update_crc(crc: u16, b: u8) -> u16 {
        (crc >> 8) ^ CRC_TABLE[(crc ^ (b as u16)) as u8 as usize]
    }

    fn append_escaping(buf: &mut Vec<u8>, b: u8) {
        if b < BOFNE {
            buf.push(b);
        } else {
            buf.push(CE);
            buf.push(b ^ XOR);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NonEscapingS101Frame {
    EmberPacket(EmberPacket),
    KeepaliveRequest,
    KeepaliveResponse,
}

impl NonEscapingS101Frame {
    pub fn len(&self) -> usize {
        4 + match self {
            Self::EmberPacket(ember_packet) => ember_packet.len(),
            Self::KeepaliveRequest | Self::KeepaliveResponse => 0,
        }
    }

    pub fn encoded_len(&self) -> usize {
        let len = self.len();
        let len_bytes = if len == 0 {
            0
        } else if len <= u8::MAX as usize {
            1
        } else if len <= u16::MAX as usize {
            2
        } else {
            panic!("max message size exceeded")
        };
        2 + len_bytes + len
    }

    pub fn encode(&self, buf: &mut [u8]) {
        buf[0] = BOFNE;
        let payload_len = self.len();
        if payload_len == 0 {
            buf[1] = 0x00;
        } else if payload_len <= u8::MAX as usize {
            buf[1] = 0x01;
            buf[2] = payload_len as u8;
        } else if payload_len <= u16::MAX as usize {
            buf[1] = 0x02;
            buf[2..4].copy_from_slice(&(payload_len as u16).to_le_bytes());
        } else {
            panic!("max message size exceeded")
        };

        let payload_start = 2 + buf[1] as usize;

        self.to_bytes(&mut buf[payload_start..]);
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        if buf.len() < self.len() {
            panic!("insufficient buffer size")
        }

        buf[0] = SLOT_IDENTIFIER;
        buf[1] = MESSAGE_TYPE;
        buf[2] = self.command_byte();
        buf[3] = VERSION;

        if let Self::EmberPacket(data) = &self {
            data.to_bytes(&mut buf[4..self.len()]);
        }
    }

    fn command_byte(&self) -> u8 {
        match self {
            Self::EmberPacket(_) => COMMAND_EMBER_PACKET,
            Self::KeepaliveRequest => COMMAND_KEEPALIVE_REQUEST,
            Self::KeepaliveResponse => COMMAND_KEEPALIVE_RESPONSE,
        }
    }

    fn decode_blocking(mut data: impl Read, buf: &mut [u8]) -> EmberResult<Option<Self>> {
        data.read_exact(&mut buf[..1])?;

        let payload_bytes = buf[0] as usize;
        if payload_bytes == 0 {
            return Ok(None);
        }

        data.read_exact(&mut buf[..payload_bytes])?;

        let mut payload_len = 0usize;
        for b in &buf[..payload_bytes] {
            payload_len = payload_len << 8;
            payload_len += *b as usize;
        }

        if payload_len == 0 {
            return Ok(None);
        }

        data.read_exact(&mut buf[..payload_len])?;

        Self::from_bytes(&buf[..payload_len]).map(Some)
    }

    async fn decode<R: AsyncRead + Unpin>(
        mut data: R,
        buf: &mut [u8],
    ) -> EmberResult<Option<Self>> {
        data.read_exact(&mut buf[..1]).await?;

        let payload_bytes = buf[0] as usize;
        if payload_bytes == 0 {
            return Ok(None);
        }

        data.read_exact(&mut buf[..payload_bytes]).await?;

        let mut payload_len = 0usize;
        for b in &buf[..payload_bytes] {
            payload_len = payload_len << 8;
            payload_len += *b as usize;
        }

        if payload_len == 0 {
            return Ok(None);
        }

        data.read_exact(&mut buf[..payload_len]).await?;

        Self::from_bytes(&buf[..payload_len]).map(Some)
    }

    fn from_bytes(buf: &[u8]) -> EmberResult<Self> {
        match buf[2] {
            COMMAND_EMBER_PACKET => EmberPacket::from_bytes(&buf[4..]).map(Self::EmberPacket),
            COMMAND_KEEPALIVE_REQUEST => Ok(Self::KeepaliveRequest),
            COMMAND_KEEPALIVE_RESPONSE => Ok(Self::KeepaliveRequest),
            it => Err(EmberError::Deserialization(format!(
                "Invalid command byte: {:#04x}",
                it
            ))),
        }
    }
}

#[cfg(test)]
mod test {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn escaping_encoding_works() {
        let packet = EmberPacket::new(Flags::SinglePacket, 2, 5, vec![0; 10]);
        let frame = EscapingS101Frame::EmberPacket(packet);
        let mut output = Vec::new();
        let mut temp = vec![0u8; 2 * frame.len()];
        frame.encode(&mut temp, &mut output);
        assert_eq!(
            vec![
                254, 0, 14, 0, 1, 192, 1, 2, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 240, 255
            ],
            output
        );
    }

    #[test]
    fn non_escaping_encoding_works() {
        let mut packet = EmberPacket::new(Flags::SinglePacket, 2, 5, vec![0; 10]);
        packet.set_flag(Flags::SinglePacket);
        let frame = NonEscapingS101Frame::EmberPacket(packet);
        let mut output = vec![0; 2 * frame.encoded_len()];
        frame.encode(&mut output);
        assert_eq!(
            vec![
                0xF8, 0x01, 0x13, 0x00, 0x0E, 0x00, 0x01, 0xC0, 0x01, 0x02, 0x05, 0x02, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            &output[..frame.encoded_len()]
        );
    }

    #[test]
    fn escaping_decoding_works() {
        let data = vec![
            254, 0, 14, 0, 1, 192, 1, 2, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 240, 255,
        ];
        let mut packet = EmberPacket::new(Flags::SinglePacket, 2, 5, vec![0; 10]);
        packet.set_flag(Flags::SinglePacket);
        let frame = EscapingS101Frame::EmberPacket(packet);
        let mut buf = vec![0; data.len()];
        assert_eq!(
            S101Frame::Escaping(frame),
            S101Frame::decode_blocking(Cursor::new(&data), &mut buf)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn non_escaping_decoding_works() {
        let data = vec![
            0xF8, 0x01, 0x13, 0x00, 0x0E, 0x00, 0x01, 0xC0, 0x01, 0x02, 0x05, 0x02, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut packet = EmberPacket::new(Flags::SinglePacket, 2, 5, vec![0; 10]);
        packet.set_flag(Flags::SinglePacket);
        let frame = NonEscapingS101Frame::EmberPacket(packet);
        let mut buf = vec![0; data.len()];
        assert_eq!(
            S101Frame::NonEscaping(frame),
            S101Frame::decode_blocking(Cursor::new(&data), &mut buf)
                .unwrap()
                .unwrap()
        );
    }
}
