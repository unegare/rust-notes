use serde::{Serialize, Deserialize, Deserializer};
use serde_json;
use mymacro::custom_derive_deserialize;

#[custom_derive_deserialize]
#[derive(Serialize, Debug)]
struct A {
    key: String,
    #[custom_derive(si.key.clone())]
    key2: String,
}

fn main() {
    println!("Hello, world!");
    let s = r#"{"key":"val"}"#;
    let a: A = serde_json::from_str(s).unwrap();
    println!("{:?}", a); // A { key: "val", key2: "val" }
}
