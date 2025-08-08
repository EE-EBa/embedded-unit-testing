#![no_std]
#![no_main]

use test2::*;

#[cfg(test)]
#[defmt_test::tests]
mod tests {
    use super::{Json, json};

    #[test]
    fn json_null1() {
        assert_eq!(json!(null), Json::Null);
    }
    
    #[test]
    fn json_null() {
        assert_eq!(json!(null), Json::Boolean(false));
    }
}
