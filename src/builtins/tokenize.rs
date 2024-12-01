//! This builtin turns C type signatures into valid identifiers

use crate::realize::{Env, Realizable};

use super::{Builtin, Macro, builtin, builtin_macro};

#[apply(builtin!)]
#[apply(builtin_macro!)]
pub fn tokenize(args: Vec<&str>, env: Vec<Env>)
{
    vec!()
}

