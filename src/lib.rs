//! The Super Cub preprocessor library.

#![feature(used_with_arg)]
#![feature(min_specialization)]

#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate pest_ast;
extern crate from_pest;

#[macro_use]
extern crate macro_rules_attribute;

#[macro_use]
extern crate linkme;

#[macro_use]
extern crate with_builtin_macros;

pub mod parser;
pub mod realize;

#[macro_use]
pub mod builtins;

use crate::parser::ast::Document;

use crate::realize::Env;

pub fn load_document(doc: &str) -> Document {
    use pest::Parser;
    use from_pest::FromPest;

    use crate::parser::cub;

    let mut pairs = cub::Parser::parse(cub::Rule::document, doc).unwrap();

    Document::from_pest(&mut pairs).unwrap()
}

/// Transform Super Cub source code (doc) into final C code, given
/// an environment.
pub fn realize(doc: &str, env: Vec<Env>) -> String {
    use crate::realize::Realizable;

    let document = load_document(doc);

    document.realize(env).to_owned()
}
