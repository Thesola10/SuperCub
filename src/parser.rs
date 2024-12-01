//! The AST parser and spec for the Super Cub preprocessor.

pub mod cub {
    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct Parser;
}

pub mod ast {
    use super::cub::Rule; // see what I did there?
    use pest::Span;

    fn span_into_str(span: Span) -> Box<str> {
        span.as_str().into()
    }

    fn span_into_int(span: Span) -> u16 {
        span.as_str().parse().unwrap()
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::argument))]
    pub struct Argument {
        #[pest_ast(outer(with(span_into_str)))]
        pub content: Box<str>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_star))]
    pub struct StarVariable;

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_num))]
    pub struct NumberVariable {
        #[pest_ast(inner(rule(Rule::var_num_digit), with(span_into_int)))]
        pub num: u16
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_base))]
    pub struct NamedVariable {
        #[pest_ast(inner(rule(Rule::ident), with(span_into_str)))]
        pub ident: Box<str>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_any))]
    pub enum Variable {
        StarVariable (StarVariable),
        NumberVariable (NumberVariable),
        NamedVariable (NamedVariable)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_string))]
    pub struct StringVariable {
        pub var: Variable
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::assign_eq))]
    pub struct Assign;

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::assign_append))]
    pub struct AssignAppend;

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::assign))]
    pub enum AssignOp {
        Assign(Assign),
        AssignAppend(AssignAppend)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_op))]
    pub struct VariableOp {
        #[pest_ast(inner(rule(Rule::ident), with(span_into_str)))]
        pub ident: Box<str>,
        pub operator: AssignOp,
        pub value: Box<CInnerChunk>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::macro_call))]
    pub struct MacroCall {
        #[pest_ast(inner(rule(Rule::ident), with(span_into_str)))]
        pub ident: Box<str>,
        pub arg_set: Vec<Argument>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::infix))]
    pub enum Infix {
        MacroCall (MacroCall),
        VariableOp (VariableOp),
        StringVariable(StringVariable),
        BareVariable(Variable),
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_incl_str))]
    pub struct LocalInclude {
        #[pest_ast(outer(with(span_into_str)))]
        content: Box<str>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_incl_sys))]
    pub struct SystemInclude {
        #[pest_ast(outer(with(span_into_str)))]
        content: Box<str>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_incl_path))]
    pub enum IncludePath {
        LocalInclude (LocalInclude),
        SystemInclude (SystemInclude)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_macro))]
    pub struct CMacro {
        #[pest_ast(outer(with(span_into_str)))]
        content: Box<str>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_stmt))]
    pub struct CStatement {
        infixes: Vec<Infix>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_block))]
    pub struct CBlock {
        infixes: Vec<Infix>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_include))]
    pub struct CInclude {
        path: IncludePath
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_inner_chunk))]
    pub enum CInnerChunk {
        Infix(Infix),
        CBlock(CBlock)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::c_chunk))]
    pub enum CChunk {
        Infix(Infix),
        CInclude (CInclude),
        CMacro (CMacro),
        CStatement (CStatement),
        CBlock (CBlock)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::decor))]
    pub struct Decorator {
        #[pest_ast(inner(rule(Rule::ident), with(span_into_str)))]
        pub ident: Box<str>,
        pub args: Vec<Argument>,
        pub target: CChunk
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::type_ident))]
    pub struct TypeIdent;

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::type_expr))]
    pub struct TypeExpr;

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::type_tt))]
    pub struct TypeTT;

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::_type))]
    pub enum ParamType {
        TypeIdent(TypeIdent),
        TypeExpr(TypeExpr),
        TypeTT(TypeTT)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::param))]
    pub struct Param {
        #[pest_ast(inner(rule(Rule::ident), with(span_into_str)))]
        pub ident: Box<str>,
        pub p_type: Option<ParamType>,
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::_match))]
    pub struct MatchRule {
        pub params: Vec<Param>,
        pub chunks: Vec<CChunk>,
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::fn_match))]
    pub struct FnMatch {
        pub rule: MatchRule
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::var_match))]
    pub struct VarMatch {
        pub rule: MatchRule
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::type_match))]
    pub struct TypeMatch {
        pub rule: MatchRule
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::any_match))]
    pub enum AnyMatch {
        Match(MatchRule),
        FnMatch(FnMatch),
        VarMatch(VarMatch),
        TypeMatch(TypeMatch)
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::macro_rules))]
    pub struct MacroRules {
        #[pest_ast(inner(rule(Rule::ident), with(span_into_str)))]
        pub ident: Box<str>,
        pub matches: Vec<AnyMatch>
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::chunk))]
    pub enum Chunk {
        CChunk (CChunk),
        MacroRules (MacroRules),
        Decorator (Decorator)
    }

    /// Root of the AST for a Super Cub document
    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::document))]
    pub struct Document {
        pub chunks: Vec<Chunk>,
        _eoi: Eoi
    }

    #[derive(Debug, Clone, FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    pub struct Eoi;
}
