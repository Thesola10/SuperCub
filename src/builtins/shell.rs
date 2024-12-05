//! This builtin invokes a shell program as a preprocessor step

use crate::{parser::ast, realize::{Env, Realizable}};

use super::{BUILTINS, Builtin, Consumer, builtin_macro, builtin_decor};

#[apply(builtin_macro!)]
pub fn shell(args: Vec<Box<str>>, env: Vec<Env>)
{
    vec!()
}

#[apply(builtin_decor!)]
pub fn shell(args: Vec<Box<str>>, target: ast::CChunk)
{
    vec!()
}
