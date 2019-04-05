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

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use serde_json::Value as JsonValue;

pub struct Client {
    state: ConnectionState,
}

pub struct Server {
    state: ConnectionState,
}

#[derive(Debug)]
#[derive(Display)]
pub struct NetError {
    description: String,
}

impl Display for NetError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(self.description.as_str());
        Ok
    }
}

impl Error for NetError {
    fn description(&self) -> &str { self.description.as_str() }
}

type NetRet<T> = Result<T, NetError>;

pub fn make_err<T>(description: String) -> Result<T, NetError> { Err(NetError { description }) }

pub fn make_str_err<T>(description: &str) -> Result<T, NetError> { make_err(description.to_owned()) }



include!(concat!(env!("OUT_DIR"), "/protocol.rs"));

pub mod ll;
pub mod pk;
pub mod handler;
