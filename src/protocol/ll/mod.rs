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
use crate::protocol::ll::disconnect::{ClientDisconnect, ServerDisconnect};
use crate::protocol::ll::login_accept::LoginAccept;
use crate::protocol::ll::login_request::LoginRequest;
use crate::protocol::ll::package::handle_package;
use crate::protocol::ll::ping::{Ping, Pong};
use crate::util::{io_error_f, VioResult};

pub mod login_request;
pub mod login_accept;
pub mod disconnect;
pub mod ping;
pub mod package;

pub fn handle_ll<H: SignalHandler, R: Read>(handler: &mut H, reader: &mut CubeReader<R>) -> VioResult {
    let id = reader.read_uint8()?;
    match id {
        login_request::LL_LOGIN_REQUEST => handler.handle_ll_login_request(LoginRequest::read(reader)?),
        login_accept::LL_LOGIN_ACCEPT => handler.handle_ll_login_accept(LoginAccept::read(reader)?),
        disconnect::LL_SERVER_DISCONNECT => handler.handle_ll_server_disconnect(ServerDisconnect::read(reader)?),
        disconnect::LL_CLIENT_DISCONNECT => handler.handle_ll_client_disconnect(ClientDisconnect::read(reader)?),
        ping::LL_PING => handler.handle_ll_ping(Ping::read(reader)?),
        ping::LL_PONG => handler.handle_ll_pong(Pong::read(reader)?),
        package::LL_PACKAGE => handle_package(handler, reader)?,
        _ => io_error_f("Unknown low-level signal ID ".to_owned() + &id.to_string())?,
    };
    Ok(())
}
