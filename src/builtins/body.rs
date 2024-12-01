//! This builtin extracts the body from a C block, for use in attribute macros

use crate::realize::{Env, Realizable};

use super::{BUILTINS, Builtin, Consumer, builtin_macro};

#[apply(builtin_macro!)]
pub fn body(args: Vec<&str>, env: Vec<Env>)
{
    vec!()
}
