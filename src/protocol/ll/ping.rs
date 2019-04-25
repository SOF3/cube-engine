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

pub const LL_PING: u8 = 0x81;

pub struct Ping {
    last_cycle: u64,
}

impl Ping {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint8(LL_PING)?;
        writer.write_uint64(self.last_cycle)?;
        Result::Ok(())
    }
    pub fn read<R: Read>(reader: &mut CubeReader<R>) -> IoResult<Self> {
        Result::Ok(Self {
            last_cycle: reader.read_uint64()?,
        })
    }
}

pub const LL_PONG: u8 = 0x82;

pub struct Pong {}

impl Pong {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint8(LL_PONG)?;
        Result::Ok(())
    }
    pub fn read<R: Read>(_reader: &mut CubeReader<R>) -> IoResult<Self> { Result::Ok(Self {}) }
}
