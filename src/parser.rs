//! The AST parser and spec for the SuperCub preprocessor

pub mod cub {
    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct Parser;
}

pub mod ast {
    use super::cub::Rule; // see what I did there?
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::infix))]
    pub struct Infix {

    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::c_chunk))]
    pub struct Chunk {
        pub infixes: Vec<Infix>
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::document))]
    pub struct Document {
        pub chunks: Vec<Chunk>
    }
}
