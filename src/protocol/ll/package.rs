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

extern crate libflate;

use std::io::{Read, Write};

use crate::io::reader::CubeReader;
use crate::io::writer::CubeWriter;
use crate::protocol::handler::SignalHandler;
use crate::protocol::pk::handle_pk;
use crate::util::VioResult;

use self::libflate::deflate::{Decoder, Encoder};

pub const LL_PACKAGE: u8 = 0xe1;

pub struct PackageWriter {
    cube: Option<CubeWriter<Encoder<Vec<u8>>>>,
}

impl PackageWriter {
    pub fn new() -> Self {
        let encoder = Encoder::new(Vec::new());
        Self {
            cube: Some(CubeWriter::new(encoder)),
        }
    }

    pub fn flush<W: Write>(&mut self, writer: &mut CubeWriter<W>) -> VioResult {
        {
            let mut cube = self.cube.take().unwrap();
            cube.ensure_complete_byte();
            cube.write_bit(false)?;
            cube.write_nop()?;
            let result = cube.target.finish().into_result()?;
            writer.write_uint8(LL_PACKAGE)?;
            writer.write_uint32(result.len() as u32)?;
            writer.write_bytes(result.as_slice())?;
        }
        let encoder = Encoder::new(Vec::new());
        let new_cube = CubeWriter::new(encoder);
        self.cube.replace(new_cube);

        Result::Ok(())
    }
}

pub fn handle_package<H: SignalHandler, R: Read>(handler: &mut H, reader: &mut CubeReader<R>) -> VioResult {
    let size = reader.read_uint32()? as usize;
    let mut buf: Vec<u8> = vec![0; size];
    reader.read_bytes(buf.as_mut_slice())?;
    let decoder = Decoder::new(buf.as_slice());
    let mut cube = CubeReader::new(decoder);
    while cube.read_bit()? {
        cube.read_nop()?;
        handle_pk(handler, &mut cube)?;
    }
    cube.read_nop()?;
    Result::Ok(())
}
