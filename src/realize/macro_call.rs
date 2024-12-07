
use super::{Realizable, Resolvable, Infixable, Env};
use crate::parser::ast;

use pest::Span;

fn apply_builtin(name: Box<str>, args: Vec<Box<str>>, env: Vec<Env>)
        -> String
{
    use crate::builtins::{BUILTINS, Builtin, Consumer};

    let mut result: Option<String> = None;

    for bi in BUILTINS {
        match bi {
            Builtin { name: nm, consumer: Consumer::Macro(c) } => {
                if nm.eq(&Box::leak(name.clone())) {
                    result = Some(c(args, env));
                    break;
                }
            },
            _ => ()
        }
    }

    result.unwrap()
}

fn apply_rules(rules: ast::MacroRules, args: Vec<Box<str>>, mut env: Vec<Env>)
        -> String
{
    let mut my_match: Option<ast::MatchRule> = None;

    for mat in rules.matches {
        match mat {
            ast::AnyMatch::Match(mr) => {
                // TODO: Also check parameter types if specified
                if mr.params.len() == args.len() {
                    my_match = Some(mr);
                    break;
                }
            },
            _ => ()
        }
    }

    match my_match {
        Some(m) => {
            for (param, arg) in m.params.iter().zip(args.iter()) {
                env.push(Env::Variable { name: param.ident.clone(), value: arg.clone() });
            }
        },
        None => ()
    }

    // TODO: Return string once CChunk is realizable

    "".to_string()
}


impl Realizable for ast::MacroCall<'_>
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        let inner_env = env.clone();
        let mut my_rule: Option<ast::MacroRules> = None;
        let my_args = self.arg_set.iter().fold(vec!(), |mut l, r| { l.push(r.content.clone()); l });

        let result: &str = "";

        for k in env.iter() {
            match k {
                Env::MacroRules(mr) => {
                    if mr.ident.eq(&self.ident) {
                        my_rule = Some(mr.clone());
                    }
                },
                _ => ()
            }
        }

        match my_rule {
            Some(mr) => apply_rules(mr, my_args, inner_env),
            None => apply_builtin(self.ident.clone(), my_args, inner_env)
        }.clone_into(&mut result.to_owned());

        result
    }
}

impl Infixable for ast::MacroCall<'_>
{
    fn get_span(&self) -> Span
    {
        self.span
    }
}

impl Resolvable for ast::MacroCall<'_> {}
