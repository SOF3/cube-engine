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

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::protocol::format::{create_field_format, FieldFormat};
use std::rc::Rc;

pub type Type = Vec<(String, Rc<FieldFormat>)>;

pub struct Types {
    pub map: LinkedHashMap<String, Type>
}

impl Types {
    pub fn new(data: &Yaml) -> Types {
        let mut map = LinkedHashMap::<String, Type>::new();
        for (name, content) in data.as_hash().unwrap() {
            let name = name.as_str().unwrap().to_owned();
            let mut typ = Type::new();
            for (field, format) in content.as_hash().unwrap() {
                let field = field.as_str().unwrap();
                let format = create_field_format(&map, field, format, format!("type {}", name).as_str());
                if let Some(format) = format {
                    let format: Rc<FieldFormat> = format.into();
                    typ.push((field.to_owned(), format));
                }
            }
            map.insert(name, typ);
        }
        Types { map }
    }
}
