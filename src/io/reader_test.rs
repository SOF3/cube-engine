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

use crate::io::reader::CubeReader;

macro_rules! make_reader {
    ($reader: ident) => {
        let mut $reader: CubeReader<&[u8]> = CubeReader::new(&[
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
        ]);
    }
}

#[test]
fn read_u8() {
    make_reader!(reader);
    assert_eq!(reader.read_uint8().unwrap(), 0x12 as u8);
}

#[test]
fn read_u8_moving() {
    make_reader!(reader);
    assert_eq!(reader.read_uint8().unwrap(), 0x12 as u8);
    assert_eq!(reader.read_uint8().unwrap(), 0x34 as u8);
}

#[test]
fn read_u16() {
    make_reader!(reader);
    assert_eq!(reader.read_uint16().unwrap(), 0x1234 as u16);
}

#[test]
fn read_u32() {
    make_reader!(reader);
    assert_eq!(reader.read_uint32().unwrap(), 0x12345678 as u32);
}

#[test]
fn read_u32_moving() {
    make_reader!(reader);
    assert_eq!(reader.read_uint32().unwrap(), 0x12345678 as u32);
    assert_eq!(reader.read_uint32().unwrap(), 0x90abcdef as u32);
}

#[test]
fn read_u64() {
    make_reader!(reader);
    assert_eq!(reader.read_uint64().unwrap(), 0x1234567890abcdef as u64);
}

#[test]
fn read_u128() {
    make_reader!(reader);
    assert_eq!(reader.read_uint128().unwrap(), 0x1234567890abcdef1234567890abcdef as u128);
}

#[test]
fn read_i8() {
    make_reader!(reader);
    assert_eq!(reader.read_int8().unwrap(), 0x12 as i8);
}

#[test]
fn read_i8_moving() {
    make_reader!(reader);
    assert_eq!(reader.read_int8().unwrap(), 0x12 as i8);
    assert_eq!(reader.read_int8().unwrap(), 0x34 as i8);
}

#[test]
fn read_i16() {
    make_reader!(reader);
    assert_eq!(reader.read_int16().unwrap(), 0x1234 as i16);
}

#[test]
fn read_i32() {
    make_reader!(reader);
    assert_eq!(reader.read_int32().unwrap(), 0x12345678 as i32);
}

#[test]
fn read_i32_moving() {
    make_reader!(reader);
    assert_eq!(reader.read_int32().unwrap(), 0x12345678 as i32);
    assert_eq!(reader.read_int32().unwrap(), 0x10abcdef as i32 + -0x8000_0000);
}

#[test]
fn read_i64() {
    make_reader!(reader);
    assert_eq!(reader.read_int64().unwrap(), 0x1234567890abcdef as i64);
}

#[test]
fn read_i128() {
    make_reader!(reader);
    assert_eq!(reader.read_int128().unwrap(), 0x1234567890abcdef1234567890abcdef as i128);
}

#[test]
fn read_f32() {
    let mut reader: CubeReader<&[u8]> = CubeReader::new(&[0x3e, 0x99, 0x99, 0x9a]);
    assert_eq!(reader.read_float32().unwrap(), 0.3 as f32);
}

#[test]
fn read_f32_inf() {
    let mut reader: CubeReader<&[u8]> = CubeReader::new(&[
        0x7f, 0x80, 0x00, 0x00,
        0xff, 0x80, 0x00, 0x00,
        0xff, 0xc0, 0x00, 0x00,
    ]);
    assert_eq!(reader.read_float32().unwrap(), std::f32::INFINITY);
    assert_eq!(reader.read_float32().unwrap(), -std::f32::INFINITY);
    assert!(reader.read_float32().unwrap().is_nan());
}

#[test]
fn read_f64() {
    let mut reader: CubeReader<&[u8]> = CubeReader::new(&[0x3f, 0xd3, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33]);
    assert_eq!(reader.read_float64().unwrap(), 0.3 as f64);
}

#[test]
fn read_f64_inf() {
    let mut reader: CubeReader<&[u8]> = CubeReader::new(&[
        0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    assert_eq!(reader.read_float64().unwrap(), std::f64::INFINITY);
    assert_eq!(reader.read_float64().unwrap(), -std::f64::INFINITY);
    assert!(reader.read_float64().unwrap().is_nan());
}

#[test]
fn read_bits() {
    make_reader!(reader);
    let expect: [bool; 8] = [false, false, false, true, false, false, true, false];
    for i in 0..8 {
        assert_eq!(reader.read_bit().unwrap(), expect[i]);
    }
}

#[test]
fn read_nop() {
    make_reader!(reader);
    let expect1: [bool; 8] = [false, false, false, true, false, false, true, false];
    let expect2: [bool; 8] = [false, false, true, true, false, true, false, false];
    for i in 0..4 {
        assert_eq!(reader.read_bit().unwrap(), expect1[i]);
    }
    reader.read_nop().unwrap();
    for i in 0..8 {
        assert_eq!(reader.read_bit().unwrap(), expect2[i]);
    }
}
#[test]
fn read_nop_safe() {
    make_reader!(reader);
    reader.read_nop().unwrap();
    assert_eq!(reader.read_uint8().unwrap(), 0x12 as u8);
}

#[test]
fn read_nibble() {
    make_reader!(reader);
    assert_eq!(reader.read_nibble().unwrap(), 0x1 as u8);
    assert_eq!(reader.read_nibble().unwrap(), 0x2 as u8);
    assert_eq!(reader.read_nibble().unwrap(), 0x3 as u8);
    assert_eq!(reader.read_nibble().unwrap(), 0x4 as u8);
}

#[test]
#[should_panic(expected = "Pointer is not at a complete nibble")]
fn read_nibble_panic() {
    make_reader!(reader);
    assert_eq!(reader.read_bit().unwrap(), false);
    reader.read_nibble().unwrap();
}

#[test]
#[should_panic(expected = "Pointer is not at a complete byte")]
fn read_nibble_u8_panic() {
    make_reader!(reader);
    assert_eq!(reader.read_nibble().unwrap(), 0x1 as u8);
    reader.read_uint8().unwrap();
}
#[test]
#[should_panic(expected = "Pointer is not at a complete byte")]
fn read_bit_u8_panic() {
    make_reader!(reader);
    assert_eq!(reader.read_bit().unwrap(), false);
    reader.read_uint8().unwrap();
}
