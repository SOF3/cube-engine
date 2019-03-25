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
extern crate libflate;

use core::borrow::BorrowMut;
use std::io::Write;

use libflate::deflate::Encoder;

use cube_engine::util::{io_error, IoResult};

use crate::common::read::StepToken;

use self::byteorder::{BigEndian, WriteBytesExt};

pub fn parse_step(tokens: &Vec<StepToken>) -> IoResult<Vec<u8>> {
    let mut buffer = Vec::<u8>::new();
    if parse_until_end(tokens, 0.borrow_mut(), &mut buffer)? { io_error("Unexpected close brace")? }
    Ok(buffer)
}

fn parse_until_end<W: Write>(tokens: &Vec<StepToken>, i: &mut usize, buffer: &mut W) -> IoResult<bool> {
    while *i < tokens.len() {
        match &tokens[*i] {
            StepToken::Byte(byte) => buffer.write_all(&[byte.clone()])?,
            StepToken::StartZlib => {
                let mut encoder = Encoder::new(Vec::new());
                parse_until_end(tokens, i, &mut encoder)?;
                let result = encoder.finish().into_result()?;
                buffer.write_all(result.as_slice())?;
            }
            StepToken::Utf8String(str) => buffer.write_all(str.as_bytes())?,
            StepToken::Float(f) => buffer.write_f32::<BigEndian>(f.clone())?,
            StepToken::Double(f) => buffer.write_f64::<BigEndian>(f.clone())?,
            StepToken::Close => { return Ok(true); }
        }
        *i += 1;
    }
    Ok(false)
}
