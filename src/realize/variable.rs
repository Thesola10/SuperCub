
use super::{span_to_ref, Env, Infixable, Realizable, Resolvable};
use crate::parser::ast;

use pest::Span;

fn find_by_name(id: String, env: Vec<Env>) -> Option<String>
{
    let mut result: Option<String> = None;

    for k in env.iter() {
        match k {
            Env::Variable { name, value } => {
                if id.eq(Box::<str>::leak(name.clone())) {
                    result = Some(Box::<str>::leak(value.clone()).to_owned());
                    break;
                }
            },
            _ => ()
        }
    }

    result
}

fn merge_numbered<'pest>(env: Vec<Env>) -> Option<String>
{
    let mut acc: String = "".to_string();

    let mut pos: u16 = 0;

    for k in env.iter() {
        match k {
            Env::Variable { name, value } => {
                if pos.to_string().eq(Box::<str>::leak(name.clone())) {
                    acc.push_str(value);
                    pos += 1;
                }
            },
            _ => ()
        }
    }

    if ! acc.is_empty() {
        Some(acc)
    } else {
        None
    }
}

impl ast::Variable<'_>
{
    fn get_value(&self, env: Vec<Env>) -> String
    {
        match self {
            ast::Variable::NamedVariable(nv) => {
                find_by_name(nv.ident.to_string(), env).expect("No such variable")
            },
            ast::Variable::NumberVariable(nv) => {
                find_by_name(nv.num.to_string(), env).expect("There is no body context to expand")
            },
            ast::Variable::StarVariable(_) => {
                merge_numbered(env).expect("There is no body context to expand")
            }
        }
    }
}

impl<'pest> Realizable<'pest> for ast::Variable<'pest>
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        let result: &str = "";

        let mut merged: String = span_to_ref(self.get_span(), env.clone());

        merged.push_str(&self.get_value(env));
        merged.clone_into(&mut result.to_owned());

        result
    }
}

impl Infixable for ast::Variable<'_>
{
    fn get_span(&self) -> Span
    {
        match self {
            ast::Variable::NamedVariable(nv) => nv.span,
            ast::Variable::NumberVariable(nv) => nv.span,
            ast::Variable::StarVariable(nv) => nv.span
        }
    }
}

// An empty impl means end of resolution tree
impl<'pest> Resolvable<'pest> for ast::Variable<'pest> {}


impl<'pest> Realizable<'pest> for ast::StringVariable<'pest>
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        let result: &str = "";

        let mut merged: String = span_to_ref(self.get_span(), env.clone());

        merged.push('"');
        merged.push_str(&self.var.get_value(env).replace(r#"""#, r#"\""#));
        merged.push('"');   //              For clarity:    "       \"
        merged.clone_into(&mut result.to_owned());

        result
    }
}

impl Infixable for ast::StringVariable<'_>
{
    fn get_span(&self) -> Span
    {
        self.span
    }
}

impl<'pest> Resolvable<'pest> for ast::StringVariable<'pest> {}
