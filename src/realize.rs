//! The realzation mechanism to turn a Super Cub AST into valid C code.

use crate::parser::ast;
use crate::builtins::Builtin;

/// AST elements which can be converted into plain text.
pub trait Realizable: Resolvable {
    /// Return plain text representing this Realizable.
    fn realize(&self, env: Vec<Env>) -> &str;
}

/// AST elements which can be converted into Realizable trees.
pub trait Resolvable {
    /// Return a tree of Realizables from interpreting this Resolvable.
    /// To traverse the tree, call resolve() on its children.
    fn resolve(&self) -> Vec<&dyn Realizable>;

    /// Return a set of environment keys for the node.
    /// In practice, this should only return MacroRules.
    fn get_env(&self) -> Vec<Env>;

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
pub enum Env {
    Variable { name: Box<str>, value: Box<str> },
    MacroRules (ast::MacroRules),
    Builtin (Box<dyn Builtin>)
}

impl Realizable for ast::Document
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        "not implemented"
    }
}

impl Resolvable for ast::Document
{
    fn resolve(&self) -> Vec<&dyn Realizable>
    {
        Vec::new()
    }

    fn get_env(&self) -> Vec<Env>
    {
        Vec::new()
    }
}
