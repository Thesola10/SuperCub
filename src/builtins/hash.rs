//! This builtin turns an arbitrary string into a valid base32 identifier

use crate::realize::{Env, Realizable};

use super::{Builtin, Macro, builtin, builtin_macro};

#[apply(builtin!)]
#[apply(builtin_macro!)]
pub fn hash(args: Vec<&str>, env: Vec<Env>)
{
    vec!()
}
