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

use std::io::Read;

use crate::io::reader::CubeReader;
use crate::protocol::handler::SignalHandler;
use crate::protocol::pk::spawn::Spawn;
use crate::util::{io_error_f, VioResult};

pub mod spawn;

pub fn handle_pk<H: SignalHandler, R: Read>(handler: &mut H, reader: &mut CubeReader<R>) -> VioResult {
    let id = reader.read_uint16()?;
    match id {
        spawn::PK_SPAWN_SPAWN => handler.handle_pk_spawn(Spawn::read(reader)?),
        _ => io_error_f("Unknown packed signal ID ".to_string() + &id.to_string())?,
    };
    Result::Ok(())
}
