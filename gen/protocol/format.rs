use yaml_rust::Yaml;

pub trait FieldFormat {}

struct SimpleFieldFormat {
    name: String,
    typ: String,
}

impl FieldFormat for SimpleFieldFormat {}

pub fn create_field_format(yaml: &Yaml) -> Box<FieldFormat> {
    if let Some(s) = yaml.as_str() {
        return create_string_field_format(s);
    }
    if let Some(s) = yaml.as_hash() {
        unimplemented!()
    }

    if let Some(s) = yaml.as_str() {
        if s.starts_with("optional:") {
            let inner = create_field_format()
        }
    }

    panic!("Unsupported field format {:?}", yaml);
}

macro_rules! simple_field_format {
    ($name: expr, $type: expr) => {
        if yaml.as_str() == Some($name){
            return Box::new(SimpleFieldFormat {
                name: $name.to_owned(),
                typ: $typ.to_owned(),
            });
        }
    };
}

fn create_string_field_format(s: &str) -> Box<FieldFormat> {
    simple_field_format!("bool", "bool");
    simple_field_format!("int8", "i8");
    simple_field_format!("int16", "i16");
    simple_field_format!("int32", "i32");
    simple_field_format!("int64", "i64");
    simple_field_format!("uint8", "u8");
    simple_field_format!("uint16", "u16");
    simple_field_format!("uint32", "u32");
    simple_field_format!("uint64", "u64");
    simple_field_format!("vint32", "i32");
    simple_field_format!("vint64", "i64");
    simple_field_format!("uvint32", "u32");
    simple_field_format!("uvint64", "u64");
    simple_field_format!("string", "String");
    panic!("Unsupported field format {:?}", yaml);
}
