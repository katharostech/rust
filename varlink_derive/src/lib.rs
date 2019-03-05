//! Macro for generating modules from a varlink interface definition
//!
//! It has the drawback, that most IDEs don't execute this and thus
//! offer no code completion.
//!
//! Examples:
//!
//! ~~~rust,ignore
//! use varlink_derive;
//!
//! varlink_derive::varlink!(org_example_ping, r#"
//! # Example service
//! interface org.example.ping
//!
//! # Returns the same string
//! method Ping(ping: string) -> (pong: string)
//! "#);
//!
//! use crate::org_example_ping::VarlinkClientInterface;
//! ~~~
//!
#![deny(
    warnings,
    unsafe_code,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    keyword_idents,
    macro_use_extern_crate,
    missing_debug_implementations,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_labels,
    unused_lifetimes,
    unstable_features,
    unreachable_pub,
    future_incompatible,
    missing_copy_implementations,
    missing_doc_code_examples,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![allow(elided_lifetimes_in_paths, missing_docs)]

extern crate proc_macro;

use proc_macro::{Span, TokenStream, TokenTree};

use varlink_generator;

/// Generates a module from a varlink interface definition
///
/// `varlink!(<modulename>, r#"<varlink interface definition>"#)`
///
/// Examples:
///
/// ~~~rust,ignore
/// use varlink_derive;
//
/// varlink_derive::varlink!(org_example_ping, r#"
/// # Example service
/// interface org.example.ping
///
/// # Returns the same string
/// method Ping(ping: string) -> (pong: string)
/// "#);
///
/// use crate::org_example_ping::VarlinkClientInterface;
/// ~~~
#[proc_macro]
pub fn varlink(input: TokenStream) -> TokenStream {
    let (name, source, _) = parse_varlink_args(input);

    expand_varlink(name, source)
}

// Parse a TokenStream of the form `name r#""#`
fn parse_varlink_args(input: TokenStream) -> (String, String, Span) {
    let mut iter = input.into_iter();
    let name = match iter.next() {
        Some(TokenTree::Ident(i)) => i.to_string(),
        Some(other) => panic!("Expected module name, found {}", other),
        None => panic!("Unexpected end of macro input"),
    };
    match iter.next() {
        Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {}
        Some(other) => panic!("Expected ',', found {}", other),
        None => panic!("Unexpected end of macro input"),
    };
    let (body_literal, span) = match iter.next() {
        Some(TokenTree::Literal(l)) => (l.to_string(), l.span()),
        Some(other) => panic!("Expected raw string literal, found {}", other),
        None => panic!("Unexpected end of macro input"),
    };
    if !body_literal.starts_with("r#\"") || !body_literal.ends_with("\"#") {
        panic!("Expected raw string literal (`r#\"...\"#`)");
    }
    let body_string = body_literal[3..body_literal.len() - 2].to_string();
    match iter.next() {
        None => {}
        Some(_) => panic!("Unexpected trailing tokens in macro"),
    }
    (name, body_string, span)
}

fn expand_varlink(name: String, source: String) -> TokenStream {
    let code = match varlink_generator::compile(source) {
        Ok(code) => code,
        Err(e) => {
            use itertools::Itertools;
            let w = e.iter().map(|e| e.to_string()).join("\n");
            panic!(w)
        }
    };

    format!("mod {} {{ {} }}", name, code).parse().unwrap()
}
