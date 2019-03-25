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

use crate::io::cube::IntPos;
use crate::io::reader::CubeReader;
use crate::io::writer::CubeWriter;
use crate::util::{IoResult, VioResult};

pub const PK_LOAD_CUBE_BATCH: u16 = 0x0102;

pub struct CubeBatchSignal {
    pos: IntPos,
    payload: [u32; 4096],
}

impl CubeBatchSignal {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint16(PK_LOAD_CUBE_BATCH)?;
        writer.write_int_pos(&self.pos)?;
        for i in 0..4096 {
            writer.write_uint32(self.payload[i])?;
        }
        Ok(())
    }

    pub fn read<R: Read>(reader: &mut CubeReader<R>) -> IoResult<CubeBatchSignal> {
        let mut pk = CubeBatchSignal {
            pos: reader.read_int_pos()?,
            payload: [0; 4096],
        };
        for i in 0..4096 {
            pk.payload[i] = reader.read_uint32()?;
        }
        Ok(pk)
    }
}
