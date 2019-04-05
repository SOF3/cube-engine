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


extern crate yaml_rust;

use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;
use std::path::Path;

use yaml_rust::YamlLoader;

use crate::protocol::fsm::FSM;
use crate::protocol::signal::SignalGroup;
use crate::protocol::types::Types;

pub struct Spec {
    pub fsm: FSM,
    pub types: Types,
    pub groups: HashMap<String, SignalGroup>,
}

impl Spec {
    pub fn new() -> Spec {
        let path = Path::new(var("CARGO_MANIFEST_DIR").expect("missing env var").as_str())
            .join("protocol/spec.yml");
        let yaml = read_to_string(path).expect("Could not read file");
        let yaml = YamlLoader::load_from_str(yaml.as_str()).expect("Failed parsing toml");
        let yaml = &yaml[0];

        let fsm = FSM::new(&yaml["FSM"]);

        let types = Types::new(&yaml["TYPES"]);

        let mut groups = HashMap::new();
        for (name, group) in yaml.as_hash().unwrap() {
            let name = name.as_str().unwrap();
            if name != "FSM" && name != "TYPES" {
                groups.insert(name.to_owned(), SignalGroup::new(&types, name, group));
            }
        }

        Spec { fsm, types, groups }
    }
}
