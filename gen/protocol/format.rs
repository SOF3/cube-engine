use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::protocol::types::Type;
use std::rc::Rc;
use syn::export::Debug;

pub trait FieldFormat: Debug {}

#[derive(Debug)]
pub struct NopFieldFormat;

impl FieldFormat for NopFieldFormat {}

#[derive(Debug)]
pub struct RefFieldFormat {
    target: Rc<FieldFormat>,
}

impl FieldFormat for RefFieldFormat {}

pub fn create_field_format(types: &LinkedHashMap<String, Type>, field_name: &str, yaml: &Yaml, field_src: &str) -> Option<Box<FieldFormat>> {
    if field_name.starts_with("_nop_") {
        return Some(Box::new(NopFieldFormat));
    }
    if field_name.starts_with("_") {
        return None;
    }

    let src_msg = format!("field {} of {}", field_name, field_src);
    let src_msg = src_msg.as_str();

    if let Some(s) = yaml.as_str() {
        return Some(create_string_field_format(s, src_msg, Some(types)));
    }
    if let Some(_) = yaml.as_hash() {
        return Some(create_hash_field_format(yaml, src_msg));
    }

    panic!("Unsupported field format {:?}", yaml);
}

#[derive(Debug)]
pub struct SimpleFieldFormat {
    name: String,
    typ: String,
}

impl FieldFormat for SimpleFieldFormat {}

#[derive(Debug)]
pub struct ByteArrayFieldFormat {
    length: usize,
}

impl FieldFormat for ByteArrayFieldFormat {}

#[derive(Debug)]
pub struct OptionalFieldFormat {
    inner: Box<FieldFormat>,
}

impl FieldFormat for OptionalFieldFormat {}

#[derive(Debug)]
pub struct JsonFieldFormat {}

impl FieldFormat for JsonFieldFormat {}

macro_rules! simple_field_format {
    ($s: expr, $name: expr, $type: expr) => {
        if $s == $name{
            return Box::new(SimpleFieldFormat {
                name: $name.to_owned(),
                typ: $type.to_owned(),
            });
        }
    };
}

fn create_string_field_format(s: &str, src_msg: &str, types: Option<&LinkedHashMap<String, Type>>) -> Box<FieldFormat> {
    if let Some(types) = types {
        if types.contains_key(s) {
            let mut properties: Vec<(String, Box<FieldFormat>)> = Vec::new();
            for (n, f) in &types[s] {
                properties.push((n.clone(), Box::new(RefFieldFormat{target: Rc::clone(f)})));
            }
            return Box::new(StructFieldFormat { properties });
        }
    }

    if s.starts_with("optional:") {
        let inner = create_string_field_format(&s["optional:".len()..], src_msg, types);
        return Box::new(OptionalFieldFormat { inner });
    }

    if s.starts_with("byte:") {
        let length = s["byte:".len()..].parse::<usize>().unwrap();
        return Box::new(ByteArrayFieldFormat { length });
    }

    if s == "string:json" {
        return Box::new(JsonFieldFormat {});
    }

    simple_field_format!(s, "bool", "bool");
    simple_field_format!(s, "nibble", "u8");
    simple_field_format!(s, "int8", "i8");
    simple_field_format!(s, "int16", "i16");
    simple_field_format!(s, "int32", "i32");
    simple_field_format!(s, "int64", "i64");
    simple_field_format!(s, "uint8", "u8");
    simple_field_format!(s, "uint16", "u16");
    simple_field_format!(s, "uint32", "u32");
    simple_field_format!(s, "uint64", "u64");
    simple_field_format!(s, "vint32", "i32");
    simple_field_format!(s, "vint64", "i64");
    simple_field_format!(s, "uvint32", "u32");
    simple_field_format!(s, "uvint64", "u64");
    simple_field_format!(s, "string", "String");
    simple_field_format!(s, "float32", "f32");
    simple_field_format!(s, "float64", "f64");

    dbg!(types);

    panic!("Unsupported field format {:?} at {}", s, src_msg);
}

#[derive(Debug)]
pub struct ArrayFieldFormat {
    size: usize,
    each: Box<FieldFormat>,
}

impl FieldFormat for ArrayFieldFormat {}

#[derive(Debug)]
pub struct PrefixFieldFormat {
    prefix: Box<FieldFormat>,
    each: Box<FieldFormat>,
}

impl FieldFormat for PrefixFieldFormat {}

#[derive(Debug)]
pub struct TailFieldFormat {
    each: Box<FieldFormat>,
}

impl FieldFormat for TailFieldFormat {}

#[derive(Debug)]
pub struct StructFieldFormat {
    properties: Vec<(String, Box<FieldFormat>)>,
}

impl FieldFormat for StructFieldFormat {}

fn create_hash_field_format(m: &Yaml, src_msg: &str) -> Box<FieldFormat> {
    let list_method = m["_list"].as_str().expect(format!("Missing _list in {}", src_msg).as_str());
    let index = list_method.find(":").expect(format!("Missing colon in {}", list_method).as_str());
    let method = &list_method[0..index];
    let data = &list_method[index + 1..];

    let each = Box::new(StructFieldFormat {
        properties: Vec::new(),
    });

    match method {
        "const" => {
            let size = data.parse::<usize>().expect("const parse error");
            Box::new(ArrayFieldFormat { size, each })
        }
        "prefix" => Box::new(PrefixFieldFormat { prefix: create_string_field_format(data, src_msg, None), each }),
        "tail" => Box::new(TailFieldFormat { each }),
        _ => panic!("Unknown list method {} at {}", method, src_msg),
    }
}
