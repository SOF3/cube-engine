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

extern crate byteorder;

use std::io::Write;

use crate::io::cube::{CubePos, CubePrecisePos, FloatPos, IntPos};
use crate::io::flex::FlexPos;
use crate::util::{io_error, VioResult};

use self::byteorder::{BigEndian, WriteBytesExt};

pub struct CubeWriter<W> {
    current_byte: u8,
    current_bit: u8,
    pub target: W,
}

/// Implements a writer supporting the data types described in spec
#[allow(dead_code)]
impl<W> CubeWriter<W> where W: Write {
    /// Creates a new CubeWriter
    pub fn new(target: W) -> CubeWriter<W> {
        CubeWriter {
            current_bit: 0,
            current_byte: 0,
            target,
        }
    }

    /// If the current byte is incomplete, fill it with zeros
    pub fn write_nop(&mut self) -> VioResult {
        if self.current_bit > 0 {
            let ret = self.target.write_all(&[self.current_byte]);
            self.current_bit = 0;
            self.current_byte = 0;
            return ret;
        }

        Ok(())
    }

    /// Panics if the current byte is incomplete
    pub fn ensure_complete_byte(&self) {
        if self.current_bit != 0 {
            panic!("Pointer is not at a complete byte")
        }
    }

    /// Writes a boolean to the target as a bit.
    /// Call `write_nop()` to skip the remaining bits of the byte
    /// before calling a nibble/full-byte write function.
    pub fn write_bit(&mut self, value: bool) -> VioResult {
        self.current_byte |= (value as u8) << (7 - self.current_bit);
        self.current_bit += 1;
        if self.current_bit >= 8 {
            let ret = self.target.write_all(&[self.current_byte]);
            self.current_bit = 0;
            self.current_byte = 0;
            return ret;
        }

        Ok(())
    }
    /// Writes a nibble to the target.
    /// Only permitted when the bit pointer is at offset `0` or `4`.
    pub fn write_nibble(&mut self, value: u8) -> VioResult {
        if self.current_bit % 4 != 0 { panic!("Pointer is not at a complete nibble") }
        for i in 0..=3 {
            self.write_bit(((value >> (3 - i)) & 1) != 0)?;
        }
        Ok(())
    }

    /// Writes 
    pub fn write_bytes(&mut self, bytes: &[u8]) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_all(bytes)
    }

    /// Writes an i8 to the target
    pub fn write_int8(&mut self, value: i8) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_all(&[value as u8])
    }
    /// Writes an i16 to the target
    pub fn write_int16(&mut self, value: i16) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_i16::<BigEndian>(value)
    }
    /// Writes an i32 to the target
    pub fn write_int32(&mut self, value: i32) -> VioResult {
        self.ensure_complete_byte();
        return self.target.write_i32::<BigEndian>(value);
    }
    /// Writes an i64 to the target
    pub fn write_int64(&mut self, value: i64) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_i64::<BigEndian>(value)
    }
    /// Writes an i128 to the target
    pub fn write_int128(&mut self, value: i128) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_i128::<BigEndian>(value)
    }
    /// Writes a u8 to the target
    pub fn write_uint8(&mut self, value: u8) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_all(&[value])
    }
    /// Writes a u16 to the target
    pub fn write_uint16(&mut self, value: u16) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_u16::<BigEndian>(value)
    }
    /// Writes a u32 to the target
    pub fn write_uint32(&mut self, value: u32) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_u32::<BigEndian>(value)
    }
    /// Writes a u64 to the target
    pub fn write_uint64(&mut self, value: u64) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_u64::<BigEndian>(value)
    }
    /// Writes a u128 to the target
    pub fn write_uint128(&mut self, value: u128) -> VioResult {
        self.ensure_complete_byte();
        self.target.write_u128::<BigEndian>(value)
    }
    /// Writes an f32 to the target
    pub fn write_float32(&mut self, value: f32) -> VioResult {
        self.ensure_complete_byte();
        return self.target.write_f32::<BigEndian>(value);
    }
    /// Writes an f64 to the target
    pub fn write_float64(&mut self, value: f64) -> VioResult {
        self.ensure_complete_byte();
        return self.target.write_f64::<BigEndian>(value);
    }

    /// Writes a string to the target with u16 length prefix
    pub fn write_string(&mut self, value: &str) -> VioResult {
        if value.len() > 0xFFFF { io_error("String is too long")?; }
        self.write_uint16(value.len() as u16)?;
        self.target.write_all(value.as_bytes())
    }
    /// Writes a string to the target with u32 length prefix
    pub fn write_string32(&mut self, value: &str) -> VioResult {
        if value.len() > 0xFFFFFFFF { io_error("String is too long")?; }
        self.write_uint16(value.len() as u16)?;
        self.target.write_all(value.as_bytes())
    }

    /// Writes an IntPos to the target
    pub fn write_int_pos(&mut self, value: &IntPos) -> VioResult {
        self.write_int32(value.x)?;
        self.write_int32(value.y)?;
        self.write_int32(value.z)?;
        Ok(())
    }
    /// Writes a FloatPos to the target
    pub fn write_float_pos(&mut self, value: &FloatPos) -> VioResult {
        self.write_float32(value.x)?;
        self.write_float32(value.y)?;
        self.write_float32(value.z)?;
        Ok(())
    }

    /// Writes a [CubePos](CubePos) to the target
    pub fn write_cube_pos(&mut self, value: &CubePos) -> VioResult {
        self.write_int_pos(&value.batch)?;
        self.write_nibble(value.local_x)?;
        self.write_nibble(value.local_y)?;
        self.write_nibble(value.local_z)?;
        self.write_nop()?;
        Ok(())
    }
    /// Writes a [CubePrecisePos](CubePrecisePos) to the target
    pub fn write_cube_precise_pos(&mut self, value: &CubePrecisePos) -> VioResult {
        self.write_int_pos(&value.cube.batch)?;
        self.write_nibble(value.cube.local_x)?;
        self.write_nibble(value.cube.local_y)?;
        self.write_nibble(value.cube.local_z)?;
        self.write_nibble(value.face)?;
        self.write_float32(value.precise_x)?;
        self.write_float32(value.precise_y)?;
        Ok(())
    }

    /// Writes a [FlexPos](FlexPos) to the target
    pub fn write_flex_pos(&mut self, value: &FlexPos) -> VioResult {
        self.write_int_pos(&value.batch)?;
        self.write_float_pos(&value.local)?;
        self.write_float32(value.yaw)?;
        self.write_float32(value.pitch)?;
        Ok(())
    }
}
