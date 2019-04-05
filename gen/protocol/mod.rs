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

#![allow(dead_code)]

extern crate heck;

use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use heck::CamelCase;
use heck::SnakeCase;

use crate::protocol::signal::SignalDirection;
use crate::protocol::spec::Spec;

mod spec;
mod fsm;
mod types;
mod signal;
mod format;

pub fn generate() {
    let spec = Spec::new();

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("protocol.rs");

    {
        let mut f = File::create(&dest_path).unwrap();
        f.write_all(b"#[derive(Debug)]\n").unwrap();
        f.write_all(b"pub enum ConnectionState {\n").unwrap();
        for (name, _) in &spec.fsm.states {
            f.write_all(b"    ").unwrap();
            f.write_all(name.to_camel_case().as_bytes()).unwrap();
            f.write_all(b",\n").unwrap();
        }
        f.write_all(b"}\n\n").unwrap();

        f.write_all(b"impl Server {\n").unwrap();
        generate_endpoint_impl(&spec, &mut f, true);
        f.write_all(b"}\n").unwrap();

        f.write_all(b"impl Client {\n").unwrap();
        generate_endpoint_impl(&spec, &mut f, true);
        f.write_all(b"}\n").unwrap();
    }
}

fn generate_endpoint_impl(spec: &Spec, f: &mut File, server: bool) {
    for (group_name, group) in &spec.groups {
        for signal in &group.signals {
            let (write, read) = match signal.direction {
                SignalDirection::ClientToServer => (!server, server),
                SignalDirection::ServerToClient => (server, !server),
                SignalDirection::Mutual => (true, true),
            };
            if write {
                for line in signal.description.lines() {
                    f.write_all(b"    /// ").unwrap();
                    f.write_all(line.as_bytes()).unwrap();
                    f.write_all(b"\n").unwrap();
                }
                f.write_all(b"    fn write_").unwrap();
                f.write_all(signal.name.to_snake_case().as_bytes()).unwrap();
                f.write_all(b"(&mut self, signal: ").unwrap();
                f.write_all(signal.name.to_camel_case().as_bytes()).unwrap();
                f.write_all(b") {\n").unwrap();
                f.write_all(b"        let state = self.state;\n").unwrap();
                f.write_all(b"        self.state = match state {\n").unwrap();
                for (_, from_state) in &spec.fsm.states {
                    for edge in &from_state.edges {
                        if edge.group_name == *group_name {
                            f.write_all(b"            ConnectionState::").unwrap();
                            f.write_all(from_state.name.to_camel_case().as_bytes()).unwrap();
                            f.write_all(b" => ConnectionState::").unwrap();
                            f.write_all(from_state.name.to_camel_case().as_bytes()).unwrap();
                            f.write_all(b",").unwrap();
                            f.write_all(b"\n").unwrap();
                        }
                    }
                }
                f.write_all(b"        };\n").unwrap();
                f.write_all(b"    }\n").unwrap();
            }
        }
    }
}
