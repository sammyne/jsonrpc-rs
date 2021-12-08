use crate::derive_rpc_service;

use proc_macro2::TokenStream;
use std::str::FromStr;

const TRAIT: &'static str = "Service";
const METHOD: &'static str = "do_request";

#[test]
fn parse_ok() {
    struct TestCase {
        input: &'static str,
        expect: String,
    }

    let new_case = |input, expect| TestCase { input, expect };

    let test_vector = vec![new_case(
        r#"X, "one" | "two" -> hello, "three" -> world"#,
        format!(
            r#"impl {} for X {{ fn {} (& mut self , method : & str) {{ match method {{ "one" | "two" => self.hello(), "three" => self.world(), _ => !todo(), }} }} }} "#,
            TRAIT, METHOD
        ),
    )];

    for (i, v) in test_vector.iter().enumerate() {
        let input = TokenStream::from_str(v.input).unwrap();
        let output = derive_rpc_service(input.into()).to_string();
        assert_eq!(v.expect, output, "#{} ---", i);
    }
}
