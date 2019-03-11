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

use crate::io::writer::CubeWriter;

#[test]
fn write_u8() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint8(0xac).unwrap();
    assert_eq!(cube.target.as_slice(), &[0xac]);
}

#[test]
fn write_u8_moving() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint8(0x12).unwrap();
    cube.write_uint8(0x34).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34]);
}

#[test]
fn write_u16() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint16(0x1234).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34]);
}

#[test]
fn write_u32() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint32(0x12345678).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn write_u32_moving() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint32(0x12345678).unwrap();
    cube.write_uint32(0x90abcdef).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
}

#[test]
fn write_u64() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint64(0x1234567890abcdef).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
}

#[test]
fn write_u128() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_uint128(0x1234567890abcdef1234567890abcdef).unwrap();
    assert_eq!(cube.target.as_slice(), &[
        0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
        0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
    ]);
}

#[test]
fn write_i8() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int8(0x2c + -0x80).unwrap();
    assert_eq!(cube.target.as_slice(), &[0xac]);
}

#[test]
fn write_i8_moving() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int8(0x12).unwrap();
    cube.write_int8(0x34).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34]);
}

#[test]
fn write_i16() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int16(0x1234).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34]);
}

#[test]
fn write_i32() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int32(0x12345678).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn write_i32_moving() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int32(0x12345678).unwrap();
    cube.write_int32(0x10abcdef + -0x8000_0000).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
}

#[test]
fn write_i64() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int64(0x1234567890abcdef).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
}

#[test]
fn write_i128() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_int128(0x1234567890abcdef1234567890abcdef).unwrap();
    assert_eq!(cube.target.as_slice(), &[
        0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
        0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
    ]);
}

#[test]
fn write_f32() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_float32(0.3).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x3e, 0x99, 0x99, 0x9a]);
}

#[test]
fn write_f32_inf() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_float32(std::f32::INFINITY).unwrap();
    cube.write_float32(-std::f32::INFINITY).unwrap();
    cube.write_float32(std::f32::NAN).unwrap();
    assert_eq!(cube.target.as_slice(), &[
        0x7f, 0x80, 0x00, 0x00,
        0xff, 0x80, 0x00, 0x00,
        0x7f, 0xc0, 0x00, 0x00,
    ]);
}

#[test]
fn write_f64() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_float64(0.3).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x3f, 0xd3, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33]);
}

#[test]
fn write_f64_inf() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_float64(std::f64::INFINITY).unwrap();
    cube.write_float64(-std::f64::INFINITY).unwrap();
    cube.write_float64(std::f64::NAN).unwrap();
    assert_eq!(cube.target.as_slice(), &[
        0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x7f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
}

#[test]
fn write_bits() {
    let mut cube = CubeWriter::new(Vec::new());
    let expect: [bool; 8] = [false, false, false, true, false, false, true, false];
    for i in 0..8 {
        cube.write_bit(expect[i]).unwrap();
    }
    assert_eq!(cube.target.as_slice(), &[0x12]);
}

#[test]
fn write_nop() {
    let mut cube = CubeWriter::new(Vec::new());
    let expect1: [bool; 8] = [false, false, false, true, false, false, true, false];
    let expect2: [bool; 8] = [false, false, true, true, false, true, false, false];
    for i in 0..4 {
        cube.write_bit(expect1[i]).unwrap();
    }
    cube.write_nop().unwrap();
    for i in 0..8 {
        cube.write_bit(expect2[i]).unwrap();
    }
    assert_eq!(cube.target.as_slice(), &[0x10, 0x34]);
}

#[test]
fn write_nop_safe() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_nop().unwrap();
    cube.write_uint8(0x12).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12]);
}

#[test]
fn write_nibble() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_nibble(0x1).unwrap();
    cube.write_nibble(0x2).unwrap();
    cube.write_nibble(0x3).unwrap();
    cube.write_nibble(0x4).unwrap();
    assert_eq!(cube.target.as_slice(), &[0x12, 0x34]);
}

#[test]
#[should_panic(expected = "Pointer is not at a complete nibble")]
fn write_nibble_panic() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_bit(true).unwrap();
    cube.write_nibble(1).unwrap();
}

#[test]
#[should_panic(expected = "Pointer is not at a complete byte")]
fn write_nibble_u8_panic() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_nibble(1).unwrap();
    cube.write_uint8(1).unwrap();
}

#[test]
#[should_panic(expected = "Pointer is not at a complete byte")]
fn write_bit_u8_panic() {
    let mut cube = CubeWriter::new(Vec::new());
    cube.write_bit(true).unwrap();
    cube.write_uint8(1).unwrap();
}
