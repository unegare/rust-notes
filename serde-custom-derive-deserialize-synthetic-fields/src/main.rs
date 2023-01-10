use std::str::FromStr;
use serde::{Deserialize, Deserializer, de::Error};
use serde_json;
use mymacro::custom_derive_deserialize;

#[custom_derive_deserialize]
#[derive(Debug)]
#[allow(dead_code)]
struct A {
    key: String,
    #[custom_derive(si.key.clone())]
    key2: String,
}

#[custom_derive_deserialize]
#[derive(Debug)]
#[allow(dead_code)]
struct B {
    key: String,
    #[custom_derive(extract::<D>(&si.key.as_str())?)]
    key2: u64,
}

fn extract<'de, D>(s: &str) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>
{
    u64::from_str(s)
        .map_err(|e| D::Error::custom(format!("{}", e)))
}

fn main() {
    println!("Hello, world!");
    { // example #1
        let s = r#"{"key":"val"}"#;
        let a: A = serde_json::from_str(s).unwrap();
        println!("{:?}", a); // A { key: "val", key2: "val" }
    }
    { // example #2 with support of error propagation/bubbling
        let s = r#"{"key":"1234"}"#;
        let b: B = serde_json::from_str(s).unwrap();
        println!("{:?}", b); // B { key: "1234", key2: 1234 }
    }
}
