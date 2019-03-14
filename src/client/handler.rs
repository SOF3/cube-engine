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

use crate::client::{Client, ClientAdapter};
use crate::protocol::handler::SignalHandler;
use crate::protocol::ll::disconnect::{ClientDisconnect, ServerDisconnect};
use crate::protocol::ll::login_accept::LoginAccept;
use crate::protocol::ll::login_request::LoginRequest;
use crate::protocol::ll::ping::{Ping, Pong};
use crate::protocol::pk::spawn::Spawn;
use crate::protocol::pk::cube_dict::CubeDict;

macro_rules! cs_only {
    () => (
        unimplemented!()
    )
}

#[allow(unused_variables)]
impl<A: ClientAdapter> SignalHandler for Client<A> {
    fn handle_ll_login_request(&mut self, signal: LoginRequest) { cs_only!() }

    fn handle_ll_login_accept(&mut self, signal: LoginAccept) {
        unimplemented!()
    }

    fn handle_ll_server_disconnect(&mut self, signal: ServerDisconnect) {
        unimplemented!()
    }

    fn handle_ll_client_disconnect(&mut self, signal: ClientDisconnect) { cs_only!() }

    fn handle_ll_ping(&mut self, signal: Ping) {
        unimplemented!()
    }

    fn handle_ll_pong(&mut self, signal: Pong) {
        unimplemented!()
    }

    fn handle_pk_spawn(&mut self, signal: Spawn) {
        unimplemented!()
    }

    fn handle_pk_cube_dict(&mut self, signal: CubeDict) {
        unimplemented!()
    }
}

