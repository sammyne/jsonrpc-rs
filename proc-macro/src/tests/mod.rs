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
        r#"X: "one" -> hello, "two" -> world"#,
        format!(
            r#"impl jsonrpc :: {} for X {{ fn {} (& mut self , method : & str , params : serde_json :: Value , metadata : & jsonrpc :: Metadata) -> jsonrpc :: Result < serde_json :: Value > {{ match method {{ "one" => {{ let request = serde_json :: from_value (params) . map_err (jsonrpc :: Error :: from) . map_err (| err | err . wrap ("unmarshal request")) ? ; match self . hello (request , metadata) {{ Ok (v) => serde_json :: to_value (v) . map_err (jsonrpc :: Error :: from) , Err (err) => Err (err) , }} }} , "two" => {{ let request = serde_json :: from_value (params) . map_err (jsonrpc :: Error :: from) . map_err (| err | err . wrap ("unmarshal request")) ? ; match self . world (request , metadata) {{ Ok (v) => serde_json :: to_value (v) . map_err (jsonrpc :: Error :: from) , Err (err) => Err (err) , }} }} , _ => Err (jsonrpc :: Error :: method_not_found ()) , }} }} }}"#,
            TRAIT, METHOD
        ),
    )];

    for (i, v) in test_vector.iter().enumerate() {
        let input = TokenStream::from_str(v.input).unwrap();
        let output = derive_rpc_service(input.into()).to_string();
        assert_eq!(v.expect, output, "#{} ---", i);
    }
}
