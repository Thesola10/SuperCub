//! This builtin invokes a shell program as a preprocessor step

use crate::{parser::ast, realize::{Env, Realizable}};

use super::{Builtin, Macro, Decorator, builtin, builtin_macro, builtin_decor};

#[apply(builtin!)]
#[apply(builtin_macro!)]
pub fn shell(args: Vec<&str>, env: Vec<Env>)
{
    vec!()
}

#[apply(builtin_decor!)]
pub fn shell(target: ast::CChunk)
{
    vec!()
}
