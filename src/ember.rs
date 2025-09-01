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

// restrictions:
// - indefinite length only for containers
// - inner and outer tags must use same length form
// - only "set" and "sequence" containers
// - document root must be a single container

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum EmBerObject {
    Container(EmBerContainer),
    Value(EmBerValue),
}

#[derive(Debug, Clone)]
pub enum EmBerContainer {
    Set(HashSet<EmBerObject>),
    Sequence(Vec<EmBerObject>),
}

#[derive(Debug, Clone)]
pub enum EmBerValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Utf8String(String),
    OctetString(Box<[u8]>),
    Null,
    RelativeObjectIdentifier,
}
