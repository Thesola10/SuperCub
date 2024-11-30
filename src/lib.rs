#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate pest_ast;
extern crate from_pest;

pub mod parser;
pub mod realize;

use crate::parser::ast::Document;

pub fn test_parse(doc: &str) {
    use pest::Parser;
    use from_pest::FromPest;
    use parser::cub;
    use parser::ast::Document;

    let mut pairs = cub::Parser::parse(cub::Rule::document, doc).unwrap();

    let _parse_tree: Document = Document::from_pest(&mut pairs).unwrap();

    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        for inner_pair in pair.into_inner() {
            println!("=> Rule:    {:?}", inner_pair.as_rule());
            println!("=> Span:    {:?}", inner_pair.as_span());
            println!("=> Text:    {}", inner_pair.as_str());
        }
    }
}


pub fn load_document(doc: &str) -> Document {
    use pest::Parser;
    use from_pest::FromPest;

    use crate::parser::cub;

    let mut pairs = cub::Parser::parse(cub::Rule::document, doc).unwrap();

    Document::from_pest(&mut pairs).unwrap()
}
