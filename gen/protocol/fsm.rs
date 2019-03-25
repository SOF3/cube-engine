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

use std::collections::HashMap;

use yaml_rust::Yaml;

pub struct FSM {
    pub states: HashMap<String, State>,
}

impl FSM {
    pub fn new(data: &Yaml) -> FSM {
        let mut fsm = FSM { states: HashMap::new() };
        for (name, tbl) in data.as_hash().unwrap() {
            let name = name.as_str().unwrap().to_owned();
            let description = tbl["_description"].as_str()
                .expect(("Missing description in state ".to_owned() + name.as_str()).as_str())
                .to_owned();
            let mut state = State {
                name: name.to_owned(),
                description,
                edges: Vec::new(),
            };
            for (group_name, to_name) in tbl.as_hash().unwrap() {
                state.edges.push(StateEdge {
                    to_name: to_name.as_str().unwrap().to_owned(),
                    group_name: group_name.as_str().unwrap().to_owned(),
                });
            }
            fsm.states.insert(name.to_owned(), state);
        }
        fsm
    }
}

pub struct State {
    pub name: String,
    pub description: String,
    pub edges: Vec<StateEdge>,
}

pub struct StateEdge {
    pub to_name: String,
    pub group_name: String,
}

