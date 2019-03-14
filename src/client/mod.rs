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

mod handler;

pub enum ClientState {
    Initial,
    LoginRequested,
    Loading,
    Spawned,
    Disconnected,
}

pub struct Client<A> {
    state: ClientState,
    adapter: A,
}

impl<A: ClientAdapter> Client<A> {
    pub fn new(adapter: A) -> Client<A> {
        Client {
            state: ClientState::Initial,
            adapter,
        }
    }
}

pub trait ClientAdapter {}
