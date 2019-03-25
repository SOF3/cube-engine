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

use yaml_rust::Yaml;
use crate::protocol::format::{FieldFormat, create_field_format};

pub struct SignalGroup {
    pub class: SignalClass,
    pub signals: Vec<Signal>,
}

impl SignalGroup {
    pub fn new(data: &Yaml) -> SignalGroup {
        let x = data["_class"].as_str().unwrap();
        let class = match x {
            "ll" => SignalClass::LowLevel,
            "pk" => SignalClass::Packed,
            _ => panic!("Unexpected class ".to_owned() + x),
        };
        let mut signals = Vec::new();
        for (name, signal) in data.as_hash().unwrap() {
            let name = name.as_str().unwrap();
            if name != "_class" {
                signals.push(Signal::new(name, signal));
            }
        }
        SignalGroup { class, signals }
    }
}

pub enum SignalClass {
    LowLevel,
    Packed,
}

pub struct Signal {
    pub name: String,
    pub description: String,
    pub direction: SignalDirection,
    pub fields: Vec<SignalField>,
}

impl Signal {
    pub fn new(name: &str, data: &Yaml) -> Signal {
        let name = name.to_owned();
        let description = data["_description"].as_str().unwrap().to_owned();
        let x = data["_direction"].as_str().unwrap();
        let direction = match x {
            "CS" => SignalDirection::ClientToServer,
            "SC" => SignalDirection::ServerToClient,
            "MT" => SignalDirection::Mutual,
            _ => panic!("Unknown direction " + x),
        };
        let mut fields = Vec::new();
        for (name, format) in data.as_hash().unwrap() {
            let name = name.as_str().unwrap().to_owned();
            fields.push(SignalField{name, format: create_field_format(format)})
        }
        Signal { name, description, direction, fields }
    }
}

pub enum SignalDirection {
    ClientToServer,
    ServerToClient,
    Mutual,
}

pub struct SignalField {
    pub name: String,
    pub format: Box<FieldFormat>,
}
