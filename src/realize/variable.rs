
use super::{Realizable, Resolvable, Infixable, Env};
use crate::parser::ast;

use pest::Span;

fn find_by_name(id: String, env: Vec<Env>) -> Option<&'static str>
{
    let mut result: Option<&str> = None;

    for k in env.iter() {
        match k {
            Env::Variable { name, value } => {
                if id.eq(Box::<str>::leak(name.clone())) {
                    result = Some(Box::<str>::leak(value.clone()));
                    break;
                }
            },
            _ => ()
        }
    }

    result
}

fn merge_numbered(env: Vec<Env>) -> Option<&'static str>
{
    let mut result: Option<&str> = None;
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

    result
}

impl Realizable for ast::Variable<'_>
{
    fn realize(&self, env: Vec<Env>) -> &str
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
impl Resolvable for ast::Variable<'_> {}


impl Realizable for ast::StringVariable<'_>
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        //TODO: Quote+escape value
        self.var.realize(env)
    }
}

impl Infixable for ast::StringVariable<'_>
{
    fn get_span(&self) -> Span
    {
        self.span
    }
}

impl Resolvable for ast::StringVariable<'_> {}
