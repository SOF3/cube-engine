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

use crate::protocol::spec::Spec;
use crate::protocol::signal::SignalDirection;

mod spec;
mod fsm;
mod signal;
mod format;

pub fn generate() {
    let spec = Spec::new();

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("protocol.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(b"#[derive(Debug)]\n").unwrap();
    f.write_all(b"pub enum ConnectionState {\n").unwrap();
    for (name, _) in spec.fsm.states {
        f.write_all(b"    ").unwrap();
        f.write_all(name.to_camel_case().as_bytes()).unwrap();
        f.write_all(b",\n").unwrap();
    }
    f.write_all(b"}\n\n").unwrap();

    f.write_all(b"impl Server {\n").unwrap();
    for (_, group) in spec.groups {
        for signal in group.signals {
            if signal.direction != SignalDirection::ClientToServer {
                f.write_all(b"    write_").unwrap();
                f.write_all(signal.name.to_snake_case().as_bytes()).unwrap();
                f.write_all(b"(&mut self, signal: ").unwrap();
                f.write_all(signal.name.to_camel_case().as_bytes()).unwrap();
                f.write_all(b") {\n").unwrap();
                f.write_all(b"        let state = self.state;\n").unwrap();
                f.write_all(b"        self.state = match state {\n").unwrap();
                for (_, state) in spec.fsm.states {
                    
                    f.write_all(b"            \n").unwrap();
                }
                f.write_all(b"        };\n").unwrap();
                f.write_all(b"    }\n").unwrap();
            }
        }
    }
    f.write_all(b"\n").unwrap();
    f.write_all(b"}\n").unwrap();

    exit(0);
}
