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

use asn1_rs::Tag;

pub mod command;
pub mod element;
pub mod function;
pub mod matrix;
pub mod node;
pub mod parameter;
pub mod primitives;
pub mod root;
pub mod streams;
pub mod template;

pub enum TagClass {
    Universal = 0x00,
    Application = 0x40,
    Context = 0x80,
    Private = 0xC0,
}

pub const fn tag(class: TagClass, number: u8) -> Tag {
    Tag((class as u32) << 8 + number as u32)
}
