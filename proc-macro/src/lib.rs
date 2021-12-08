extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Expr;

#[proc_macro]
pub fn rpcize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_rpc_service(TokenStream::from(input)).into()
}

fn derive_rpc_service(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let (raw_type_name, raw_callback_mappings) = input_str
        .split_once(':')
        .expect("split type name and callback mappings");

    let type_ = syn::parse_str::<Ident>(raw_type_name.trim()).expect("parse type name");

    let callbacks = raw_callback_mappings
        .split(",")
        .map(|v| v.trim())
        .filter(|v| v.len() != 0)
        .map(parse_callback_mapping)
        .enumerate()
        .map(|(i, v)| v.map_err(|err| format!("#{} parse error: {}", i, err)))
        .collect::<Result<Vec<_>, String>>()
        .expect("parse callback mappings");

    must_generate_derive_code(type_, callbacks.as_slice())
}

fn parse_callback_mapping(raw: &str) -> Result<TokenStream, String> {
    let mut tokens = raw.splitn(2, "->");

    let (patterns, actions) = {
        let a = tokens
            .next()
            .map(|v| v.trim())
            .ok_or_else(|| "miss patterns/methods")?;

        match tokens.next().map(|v| v.trim()) {
            Some(v) => (a.to_string(), v),
            None => (format!(r#""{}""#, a), a),
        }
    };

    let patterns = syn::parse_str::<Expr>(&patterns).map_err(|v| v.to_string())?;
    let actions = syn::parse_str::<Ident>(actions).map_err(|v| v.to_string())?;

    let out = quote! {
        #patterns => {
            let request = serde_json::from_value(params)
                .map_err(jsonrpc::Error::from)
                .map_err(|err| err.wrap("unmarshal request"))?;
            match self.#actions(request, metadata) {
                Ok(v) => serde_json::to_value(v).map_err(jsonrpc::Error::from),
                Err(err) => Err(err),
            }
        },
    };

    Ok(out)
}

fn must_generate_derive_code(type_: Ident, callbacks: &[TokenStream]) -> TokenStream {
    let out = quote! {
        impl jsonrpc::Service for #type_ {
            fn do_request(&mut self, method: &str, params: serde_json::Value, metadata: &jsonrpc::Metadata) -> jsonrpc::Result<serde_json::Value> {
                match method {
                    #(#callbacks)*
                    _ => Err(jsonrpc::Error::method_not_found()),
                }
            }
        }
    };

    out
}

#[cfg(test)]
mod tests;
