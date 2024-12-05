//! This builtin turns an arbitrary string into a valid base32 identifier

use crate::realize::{Env, Realizable};

use super::{BUILTINS, Builtin, Consumer, builtin_macro};

#[apply(builtin_macro!)]
pub fn hash(args: Vec<Box<str>>, env: Vec<Env>)
{
    "".to_string()
}
