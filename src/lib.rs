#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate pest_ast;
extern crate from_pest;

mod parser;

pub fn test_parse(doc: &str) {
    use pest::Parser;
    use parser::cub;

    let pairs = cub::Parser::parse(cub::Rule::document, doc).unwrap();

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

