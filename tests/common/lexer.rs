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

extern crate hex;
extern crate lazy_static;
extern crate regex;

use cube_engine::util::{VioResult, io_error, io_error_f, IoResult};
use crate::common::read::StepToken;
use std::str::{FromStr, Chars};
use regex::Regex;

pub fn lex_step_line(line: &str, buffer: &mut Vec<StepToken>) -> VioResult {
    lazy_static::lazy_static! {
        static ref HEX_PAIR: Regex = Regex::new("^[0-9a-fA-f]{2}").unwrap();
    }
    let mut chars = line.chars();
    loop {
        let next = match chars.next() {
            Some(c) => c,
            None => break,
        };
        if next == ' ' || next == '\t' || next == ',' { continue; }
        if '0' <= next && next <= '9' || 'a' <= next && next <= 'f' {
            let next2 = match chars.next() {
                Some(c) if '0' <= c && c <= '9' || 'a' <= c && c <= 'f' => c,
                _ => io_error("Unexpected singleton nibble")?,
            };
            let pair = (&[next, next2]).iter().collect::<String>();
            let byte = match hex::decode(pair) {
                Err(e) => io_error_f(e.to_string())?,
                Ok(u) => {
                    assert_eq!(u.len(), 1);
                    u[0]
                }
            };
            buffer.push(StepToken::Byte(byte));
            continue;
        }
        match next {
            '}' => buffer.push(StepToken::Close),
            ';' => { break; }
            'Z' | '#' | 'F' | 'D' => {
                let next2 = chars.next();
                match next2 {
                    Some(c) if c == '{' => (),
                    _ => {
                        let mut message = "Unexpected token '".to_owned();
                        message.push(next);
                        match next2 {
                            Some(c) => message.push(c),
                            None => (),
                        }
                        message.push('\'');
                        io_error_f(message)?
                    }
                };
                match next {
                    'Z' => { buffer.push(StepToken::StartZlib) }
                    '#' => {
                        scan_utf8(&mut chars, buffer)?;
                    }
                    'F' => { buffer.push(StepToken::Float(scan_float::<f32>(&mut chars)?)); }
                    'D' => { buffer.push(StepToken::Double(scan_float::<f64>(&mut chars)?)); }
                    _ => panic!("Unexpected value")
                }
            }
            _ => {
                let mut message = "Unexpected token ".to_owned();
                message.push(next);
                io_error_f(message)?
            }
        };
    }
    Result::Ok(())
}

fn scan_utf8(chars: &mut Chars, buffer: &mut Vec<StepToken>) -> VioResult {
    let mut vec = Vec::<char>::new();
    loop {
        match chars.next() {
            Some(c) if c == '\\' => match chars.next() {
                Some(c2) => vec.push(c2),
                None => io_error("Unexpected end of line while parsing UTF-8 literal")?,
            },
            Some(c) if c == '}' => {
                buffer.push(StepToken::Utf8String(vec.iter().collect()));
                return Result::Ok(());
            }
            Some(c) => vec.push(c),
            None => io_error("Unexpected end of line while parsing UTF-8 literal")?,
        }
    }
}

fn scan_float<F: FromStr>(chars: &mut Chars) -> IoResult<F> {
    let mut read = Vec::<char>::new();
    loop {
        match chars.next() {
            Some(c) if '0' <= c && c <= '9' || c == '.' || c == '-' || c == 'e' => read.push(c),
            Some(c) if c == '}' => break,
            Some(c) if c == ' ' || c == '\t' => continue,
            Some(_) => io_error("Unexpected token while parsing float literal")?,
            None => io_error("Unexpected end of line while parsing float literal")?,
        }
    }
    let number = match read.iter().collect::<String>().parse::<F>(){
        Ok(n) => n,
        Err(_) => io_error::<F>("Error parsing float")?,
    };
    Result::Ok(number)
}
