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

pub const LL_LOGIN_REQUEST: u8 = 0x21;

pub struct LoginRequest {
    major_protocol: u32,
    minor_protocol: u32,
    username: String,
    user_id: [u8; 20],
    language: String,
    sys_info: String,
}

impl LoginRequest {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint8(LL_LOGIN_REQUEST)?;
        writer.write_uint32(self.major_protocol)?;
        writer.write_uint32(self.minor_protocol)?;
        writer.write_string(self.username.as_str())?;
        writer.write_bytes(&self.user_id)?;
        writer.write_string(self.language.as_str())?;
        writer.write_string(self.sys_info.as_str())?;
        Ok(())
    }
    pub fn read<R: Read>(reader: &mut CubeReader<R>) -> IoResult<Self> {
        Ok(Self {
            major_protocol: reader.read_uint32()?,
            minor_protocol: reader.read_uint32()?,
            username: reader.read_string()?,
            user_id: {
                let mut array = [0; 20];
                reader.read_bytes(&mut array)?;
                array
            },
            language: reader.read_string()?,
            sys_info: reader.read_string()?,
        })
    }
}
