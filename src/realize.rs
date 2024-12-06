//! The realzation mechanism to turn a Super Cub AST into valid C code.

use crate::parser::ast;

use pest::Span;

pub mod macro_call;
pub mod variable;
pub mod decorator;

/// AST elements which can be converted into plain text.
pub trait Realizable: Resolvable {
    /// Return plain text representing this Realizable.
    fn realize(&self, env: Vec<Env>) -> &str;
}

/// AST elements which are meant to be carved out by a Span.
pub trait Infixable {
    /// Return span representing this Infixable.
    fn get_span(&self) -> Span;
}

/// AST elements which can be converted into Realizable trees.
pub trait Resolvable {
    /// Return a tree of Realizables from interpreting this Resolvable.
    /// To traverse the tree, call resolve() on its children.
    fn resolve(&self) -> Vec<&dyn Realizable>
    { vec!() }

    /// Return a set of environment keys for the node.
    /// In practice, this should only return MacroRules.
    fn get_env(&self) -> Vec<Env>
    { vec!() }

    /// Return a set of environment keys in the tree.
    /// In practice, this should only return MacroRules.
    fn gather_env(&self) -> Vec<Env>
    {
        let mut target_vec = self.get_env();

        for m in self.resolve() {
            target_vec.append(&mut m.gather_env());
        }

        target_vec
    }
}

/// An object representing an environment key susceptible to alter
/// the Realization process.
#[derive(Clone)]
pub enum Env<'pest> {
    Variable { name: Box<str>, value: Box<str> },
        // TODO: should we use Vec<Span> instead to remember assignment locs?
    MacroRules (ast::MacroRules<'pest>),
    FileName (Box<str>)
}

/// A small generator function for #line directives in the document
pub fn span_to_ref(span: Span, env: Vec<Env>) -> String
{
    let line_no = span.start_pos().line_col().0;
    let mut name: &str = "(builtin)";

    for key in env {
        match key {
            Env::FileName(fi) => { name = Box::leak(fi); break; },
            _ => ()
        }
    }

    format!("\n#line {} \"{}\"\n", line_no, name)
}

impl Realizable for ast::Document<'_>
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        "not implemented"
    }
}

impl Resolvable for ast::Document<'_>
{
    fn resolve(&self) -> Vec<&dyn Realizable>
    {
        vec!()
    }

    fn get_env(&self) -> Vec<Env>
    {
        vec!()
    }
}
