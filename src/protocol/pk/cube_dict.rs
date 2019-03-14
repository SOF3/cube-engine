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

pub const PK_LOAD_CUBE_DICT: u16 = 0x0101;

pub struct CubeDef {
    id: u32,
    name: String,
}

pub struct CubeDict {
    size: u32,
    defs: Vec<Box<CubeDef>>,
}

impl CubeDict {
    pub fn write<W: Write>(&self, writer: &mut CubeWriter<W>) -> VioResult {
        writer.write_uint16(PK_LOAD_CUBE_DICT)?;
        writer.write_uint32(self.size)?;
        for i in 0..self.size {
            let def = &self.defs[i as usize];
            writer.write_uint32(def.id)?;
            writer.write_string(def.name.as_str())?;
            if !def.name.starts_with("CubePump.") {
                // TODO write cube model
            }
        }
        Result::Ok(())
    }

    pub fn read<R: Read>(reader: &mut CubeReader<R>) -> IoResult<CubeDict> {
        let size = reader.read_uint32()?;
        let mut defs = Vec::<Box<CubeDef>>::new();
        for _ in 0..size {
            let id = reader.read_uint32()?;
            let name = reader.read_string()?;
            if !name.starts_with("CubePump.") {
                // TODO write cube model
            }
            let def = CubeDef { id, name };
            defs.push(Box::new(def));
        }
        Result::Ok(CubeDict { size, defs })
    }
}
