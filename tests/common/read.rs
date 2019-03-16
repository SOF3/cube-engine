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

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cube_engine::util::{io_error, IoResult, io_error_f};

use crate::common::{Step, Test};
use crate::common::lexer::lex_step_line;
use crate::common::parser::parse_step;
use crate::common::preprocess::preprocess_test_file;

pub enum StepToken {
    Byte(u8),
    StartZlib,
    Utf8String(String),
    Float(f32),
    Double(f64),
    Close,
}

impl Step<StepToken> {
    fn into_u8(self) -> IoResult<Step<u8>> {
        Result::Ok(Step {
            from_server: self.from_server,
            buffer: parse_step(&self.buffer)?,
        })
    }
}

pub fn tests_dir() -> PathBuf { Path::new("tests/hex").to_path_buf() }

pub fn lib_dir() -> PathBuf { tests_dir().join("lib") }

pub fn read_tests() -> IoResult<HashMap<String, Test>> {
    let mut ret = HashMap::<String, Test>::new();
    for file in tests_dir().read_dir()? {
        let file = file?.path();
        if !file.is_file() { continue; }
        let name = match (match file.file_name() {
            Some(n) => n,
            None => io_error("Bad filename")?,
        }).to_str() {
            Some(n) => n,
            None => io_error("Bad filename")?,
        };
        if name.ends_with(".txt") {
            let test = read_test_file(file)?;
            ret.insert(test.name.to_owned().replace("-", " "), test);
        }
    }
    Result::Ok(ret)
}

fn read_test_file(file: PathBuf) -> IoResult<Test> {
    let name = match file.file_name() {
        Some(inner) => match inner.to_str() {
            Some(value) => value,
            None => io_error("Bad filename")?
        },
        None => io_error("Bad filename")?
    };

    let mut test = Test {
        name: name[0..name.len() - 4].to_owned(),
        steps: Vec::new(),
    };

    let lines = preprocess_test_file(file)?;
    dbg!(&lines);
    let mut current_step: Option<Step<StepToken>> = None;
    let mut i: usize = 0;
    loop {
        if i >= lines.len() { break; }
        let line = &lines[i];
        let mut line = line.as_str();
        if line.starts_with("<") || line.starts_with(">") {
            let old = current_step.take();
            match old {
                Some(step) => test.steps.push(step.into_u8()?),
                None => (),
            };
            current_step.replace(Step::<StepToken> {
                from_server: line.starts_with("<"),
                buffer: Vec::new(),
            });
            line = line[1..line.len()].trim();
        }

        match &mut current_step {
            Some(step) => lex_step_line(&line, &mut step.buffer)?,
            None => io_error_f("Encountered buffer line without direction, on line \"".to_owned() + line + "\"")?,
        };
        i += 1;
    }
    let old = current_step.take();
    match old {
        Some(step) => test.steps.push(step.into_u8()?),
        None => (),
    }
    Result::Ok(test)
}

