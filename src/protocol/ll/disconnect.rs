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

use std::io::{Read, Write};

use crate::io::reader::CubeReader;
use crate::io::writer::CubeWriter;
use crate::util::{IoResult, VioResult};

pub const LL_SERVER_DISCONNECT: u8 = 0x61;

pub struct ServerDisconnect {
    reason: String,
    rejoin: bool,
}

impl ServerDisconnect {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint8(LL_SERVER_DISCONNECT)?;
        writer.write_string(self.reason.as_str())?;
        writer.write_bit(self.rejoin)?;
        Result::Ok(())
    }
    pub fn read<R: Read>(reader: &mut CubeReader<R>) -> IoResult<Self> {
        Result::Ok(Self {
            reason: reader.read_string()?,
            rejoin: false,
        })
    }
}

pub const LL_CLIENT_DISCONNECT: u8 = 0x62;

pub struct ClientDisconnect {}

impl ClientDisconnect {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint8(LL_CLIENT_DISCONNECT)?;
        Result::Ok(())
    }
    pub fn read<R: Read>(_reader: &mut CubeReader<R>) -> IoResult<Self> { Result::Ok(Self {}) }
}
