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

use std::error::Error;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::io::cube::{CubePos, CubePrecisePos, FloatPos, IntPos};
use crate::io::flex::FlexPos;
use crate::util::{IoResult, make_io_error, VioResult};

pub struct CubeReader<R> {
    current_bit: u8,
    current_byte: u8,
    source: R,
}

macro_rules! read_bytes {
    ($self: ident, $size: expr) => (
        {
            let mut buf: [u8; $size] = [0; $size];
            $self.source.read_exact(&mut buf)?;
            buf
        }
    )
}

/// Implements a reader supporting the data types described in spec
impl<R> CubeReader<R> where R: Read {
    /// Creates a new CubeReader
    pub fn new(source: R) -> CubeReader<R> {
        CubeReader {
            current_bit: 0,
            current_byte: 0,
            source,
        }
    }

    /// If the current byte is incomplete, skip the remaining bits
    pub fn read_nop(&mut self) -> VioResult {
        if self.current_bit > 0 {
            self.current_bit = 0;
            self.current_byte = 0;
        }
        Result::Ok(())
    }

    fn ensure_complete_byte(&self) {
        if self.current_bit != 0 {
            panic!("Pointer is not at a complete byte")
        }
    }

    /// Reads the next bit as a boolean.
    /// Call `read_nop()` to skip the remaining bits of the byte
    /// before calling a nibble/full-byte read function.
    pub fn read_bit(&mut self) -> IoResult<bool> {
        if self.current_bit == 0 {
            self.current_byte = read_bytes!(self, 1)[0];
        }
        let ret = (self.current_byte & (1 << (7 - self.current_bit))) > 0;
        self.current_bit += 1;
        self.current_bit &= 7;
        Result::Ok(ret)
    }
    /// Reads the next nibble as a boolean.
    /// The bit pointer must be at offset `0` or `4`.
    pub fn read_nibble(&mut self) -> IoResult<u8> {
        if self.current_bit & 3 > 0 {
            panic!("Pointer is not at a complete nibble")
        }
        if self.current_bit == 0 {
            self.current_byte = read_bytes!(self, 1)[0];
        }
        let ret = (self.current_byte >> (4 - self.current_bit)) & 0x0F;
        self.current_bit ^= 4;
        self.current_bit &= 7;
        Result::Ok(ret)
    }

    /// Reads `buf.len()` bytes from the source into `buf`
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> VioResult {
        self.ensure_complete_byte();
        self.source.read_exact(buf)?;
        Result::Ok(())
    }

    /// Reads an i8 from the source
    pub fn read_int8(&mut self) -> IoResult<i8> {
        self.ensure_complete_byte();
        Result::Ok(read_bytes!(self, 1)[0] as i8)
    }
    /// Reads an i16 from the source
    pub fn read_int16(&mut self) -> IoResult<i16> {
        self.ensure_complete_byte();
        self.source.read_i16::<BigEndian>()
    }
    /// Reads an i32 from the source
    pub fn read_int32(&mut self) -> IoResult<i32> {
        self.ensure_complete_byte();
        self.source.read_i32::<BigEndian>()
    }
    /// Reads an i64 from the source
    pub fn read_int64(&mut self) -> IoResult<i64> {
        self.ensure_complete_byte();
        self.source.read_i64::<BigEndian>()
    }
    /// Reads an i128 from the source
    pub fn read_int128(&mut self) -> IoResult<i128> {
        self.ensure_complete_byte();
        self.source.read_i128::<BigEndian>()
    }
    /// Reads a u8 from the source
    pub fn read_uint8(&mut self) -> IoResult<u8> {
        self.ensure_complete_byte();
        Result::Ok(read_bytes!(self, 1)[0])
    }
    /// Reads a u16 from the source
    pub fn read_uint16(&mut self) -> IoResult<u16> {
        self.ensure_complete_byte();
        self.source.read_u16::<BigEndian>()
    }
    /// Reads a u32 from the source
    pub fn read_uint32(&mut self) -> IoResult<u32> {
        self.ensure_complete_byte();
        self.source.read_u32::<BigEndian>()
    }
    /// Reads a u64 from the source
    pub fn read_uint64(&mut self) -> IoResult<u64> {
        self.ensure_complete_byte();
        self.source.read_u64::<BigEndian>()
    }
    /// Reads a u128 from the source
    pub fn read_uint128(&mut self) -> IoResult<u128> {
        self.ensure_complete_byte();
        self.source.read_u128::<BigEndian>()
    }
    // Reads an f32 from the source
    pub fn read_float32(&mut self) -> IoResult<f32> {
        self.ensure_complete_byte();
        self.source.read_f32::<BigEndian>()
    }
    // Reads an f64 from the source
    pub fn read_float64(&mut self) -> IoResult<f64> {
        self.ensure_complete_byte();
        self.source.read_f64::<BigEndian>()
    }

    /// Reads a string from the source with u16 length prefix
    pub fn read_string(&mut self) -> IoResult<String> {
        self.ensure_complete_byte();
        let size = self.read_uint16()? as usize;
        let mut vec = vec![0; size];
        self.source.read_exact(vec.as_mut_slice())?;
        let string = String::from_utf8(vec)
            .map_err(|err| make_io_error(err.description()))?;
        return Result::Ok(string);
    }
    /// Reads a string from the source with u32 length prefix
    pub fn read_string32(&mut self) -> IoResult<String> {
        self.ensure_complete_byte();
        let size = self.read_uint32()? as usize;
        let mut vec = vec![0; size];
        self.source.read_exact(vec.as_mut_slice())?;
        let string = String::from_utf8(vec)
            .map_err(|err| make_io_error(err.description()))?;
        return Result::Ok(string);
    }

    /// Reads an IntPos from the source
    pub fn read_int_pos(&mut self) -> IoResult<IntPos> {
        Result::Ok(IntPos {
            x: self.read_int32()?,
            y: self.read_int32()?,
            z: self.read_int32()?,
        })
    }
    /// Reads a FloatPos from the source
    pub fn read_float_pos(&mut self) -> IoResult<FloatPos> {
        Result::Ok(FloatPos {
            x: self.read_float32()?,
            y: self.read_float32()?,
            z: self.read_float32()?,
        })
    }

    /// Reads a [CubePos](CubePos) from the source
    pub fn read_cube_pos(&mut self) -> IoResult<CubePos> {
        let ret = CubePos {
            batch: self.read_int_pos()?,
            local_x: self.read_nibble()?,
            local_y: self.read_nibble()?,
            local_z: self.read_nibble()?,
        };
        self.read_nop()?;
        Result::Ok(ret)
    }
    /// Reads a [CubePrecisePos](CubePrecisePos) from the source
    pub fn read_cube_precise_pos(&mut self) -> IoResult<CubePrecisePos> {
        Result::Ok(CubePrecisePos {
            cube: CubePos {
                batch: self.read_int_pos()?,
                local_x: self.read_nibble()?,
                local_y: self.read_nibble()?,
                local_z: self.read_nibble()?,
            },
            face: self.read_nibble()?,
            precise_x: self.read_float32()?,
            precise_y: self.read_float32()?,
        })
    }

    /// Writes a [FlexPos](FlexPos) to the target
    pub fn read_flex_pos(&mut self) -> IoResult<FlexPos> {
        Result::Ok(FlexPos {
            batch: self.read_int_pos()?,
            local: self.read_float_pos()?,
            yaw: self.read_float32()?,
            pitch: self.read_float32()?,
        })
    }
}
