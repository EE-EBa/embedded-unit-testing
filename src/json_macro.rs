use alloc::{string::String, vec::Vec};

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
}

#[macro_export]
macro_rules! json {
    (null) => {
        Json::Null
    };
}

