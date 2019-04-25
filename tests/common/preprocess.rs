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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use cube_engine::util::{IoResult, io_error_f};

use crate::common::read::lib_dir;

pub fn preprocess_test_file(file: PathBuf) -> IoResult<Vec<String>> {
    let fs = match File::open(&file){
        Ok(f) => f,
        Err(e) => io_error_f("Error pre-processing file ".to_owned() + file.to_str().unwrap_or("<unknown>") + ": " + e.to_string().as_str())?,
    };

    let mut lines = Vec::<String>::new();

    for line in BufReader::new(fs).lines() {
        let line = line?;
        let line = line.trim();
        if line.starts_with(";") || line.is_empty() { continue; }

        if line.starts_with("+") || line.starts_with("+") {
            let path = lib_dir().join(line[1..line.len()].trim().to_owned() + ".txt");
            for inner in preprocess_test_file(path)? {
                lines.push(inner);
            }
            continue;
        }

        lines.push(line.to_owned());
    }

    Result::Ok(lines)
}
