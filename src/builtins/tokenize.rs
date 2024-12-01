//! This builtin turns C type signatures into valid identifiers

use crate::realize::{Env, Realizable};

use super::{BUILTINS, Builtin, Consumer, builtin_macro};

#[apply(builtin_macro!)]
pub fn tokenize(args: Vec<&str>, env: Vec<Env>)
{
    vec!()
}

