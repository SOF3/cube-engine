/*
 * cube-engine
 *
 * Copyright (C) 2019 SOFe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate serde_json;

use std::io::{Read, Write};

use serde_json::Value as JsonValue;

pub struct Client<W: Write, R: Read> {
    state: ConnectionState,
    write: W,
    read: R,
}

pub struct Server<W: Write, R: Read> {
    state: ConnectionState,
    write: W,
    read: R,
}

#[derive(Debug)]
pub struct NetError {
    pub description: String,
}

impl NetError {
    pub fn new(description: String) -> NetError { NetError { description } }
}

type NetRet<T> = Result<T, NetError>;

pub fn make_err<T>(description: String) -> Result<T, NetError> { Err(NetError { description }) }

pub fn make_str_err<T>(description: &str) -> Result<T, NetError> { make_err(description.to_owned()) }

include!(concat!(env!("OUT_DIR"), "/protocol.rs"));

pub mod ll;
pub mod pk;
pub mod handler;
