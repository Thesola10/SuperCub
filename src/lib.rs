use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CubParser;

pub fn test_parse(doc: &str) {
    let pairs = CubParser::parse(Rule::document, doc).unwrap();

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

