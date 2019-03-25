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

use std::fmt::{Display, Formatter, Error};

pub mod read;
pub mod preprocess;
pub mod lexer;
pub mod parser;

pub struct Test {
    pub name: String,
    pub steps: Vec<Step<u8>>,
}

impl Display for Test {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Test {}: {{\n", self.name)?;
        for step in &self.steps {
            write!(f, "  {},", step)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

pub struct Step<S> {
    pub from_server: bool,
    pub buffer: Vec<S>,
}

impl Display for Step<u8> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let buf = hex::encode(&self.buffer);
        write!(f, "from_server: {}, buffer:\n{}\n", self.from_server, buf)
    }
}
