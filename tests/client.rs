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

extern crate cube_engine;
extern crate websocket;

//use websocket::client::async::ClientNew;
//use websocket::ClientBuilder;
//use websocket::stream::async::TcpStream;

mod common;

#[test]
fn test_hex() {
    let tests = common::read::read_tests().expect("Error reading tests");
//    let mut port = 8765;
    for (name, test) in tests {
//        let mut client = ClientBuilder::new(("ws://127.0.0.1:".to_owned() + port.to_string().as_str()).as_str())
//            .expect("Failed creating client");
//        client.add_protocol("cube-pump");
//        test.run_client(client.async_connect_insecure());
        // TODO
        println!("{}", test);
//        port += 1;
    }
}
