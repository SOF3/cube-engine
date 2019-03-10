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

use std::io::{Error, ErrorKind};

pub type IoResult<T> = Result<T, Error>;
pub type VioResult = IoResult<()>;

pub fn make_io_error(desc: &str) -> Error {
    Error::new(ErrorKind::Other, desc)
}

pub fn io_error<T>(desc: &str) -> Result<T, Error> {
    Result::Err(make_io_error(desc))
}

pub fn io_error_f<T>(desc: String) -> Result<T, Error> {
    Result::Err(make_io_error(desc.as_str()))
}
