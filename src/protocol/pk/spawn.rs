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

use crate::io::flex::FlexPos;
use crate::io::reader::CubeReader;
use crate::io::writer::CubeWriter;
use crate::util::{IoResult, VioResult};

pub const PK_SPAWN_SPAWN: u16 = 0x0201;

pub struct SpawnSignal {
    pos: FlexPos,
}

impl SpawnSignal {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint16(PK_SPAWN_SPAWN)?;
        writer.write_flex_pos(&self.pos)?;
        Result::Ok(())
    }

    pub fn read<R: Read>(reader: &mut CubeReader<R>) -> IoResult<SpawnSignal> {
        Result::Ok(SpawnSignal {
            pos: reader.read_flex_pos()?,
        })
    }
}
