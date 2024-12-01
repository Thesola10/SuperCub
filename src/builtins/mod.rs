//! Set of built-in macros in Super Cub.

#![macro_escape]

use crate::parser::ast;

use crate::realize::{Realizable, Env};

pub mod tokenize;
pub mod body;
pub mod shell;
pub mod hash;


/// Built-in macro that can be used as a function-like call
pub trait Macro: Builtin {
    fn realize(&self, args: Vec<&str>, env: Vec<Env>) -> Vec<&dyn Realizable>;
}

/// Built-in macro that can be used as a decorator
pub trait Decorator: Builtin {
    fn resolve(&self, target: ast::CChunk) -> Vec<&dyn Realizable>;
}

/// Common trait for builtins
pub trait Builtin {
    fn get_name(&self) -> &str;
}

#[macro_export]
macro_rules! builtin {(
    $(#[$fn_meta:meta])*
    $fn_vis:vis
    fn $NAME:ident ($($param:ident : $type:ty),*) $body:block
) => (
    $fn_vis struct $NAME {}

    impl Builtin for $NAME {
        fn get_name(&self) -> &str { stringify!($NAME) }
    }

    $(#[$fn_meta])*
    $fn_vis
    fn $NAME ($($param : $type),*) $body
)}

#[macro_export]
macro_rules! builtin_macro {(
    $(#[$fn_meta:meta])*
    $fn_vis:vis
    fn $NAME:ident ($($param:ident : $type:ty),*) $body:block
) => (
    impl Macro for $NAME {
        $(#[$fn_meta])*
        fn realize(&self, $($param : $type),*) -> Vec<&dyn Realizable> $body
    }
)}

#[macro_export]
macro_rules! builtin_decor {(
    $(#[$fn_meta:meta])*
    $fn_vis:vis
    fn $NAME:ident ($($param:ident : $type:ty),*) $body:block
) => (
    impl Decorator for $NAME {
        $(#[$fn_meta])*
        fn resolve(&self, $($param : $type),*) -> Vec<&dyn Realizable> $body
    }
)}

pub(crate) use builtin;
pub(crate) use builtin_macro;
pub(crate) use builtin_decor;


pub fn get_all_builtins() -> Vec<Box<dyn Builtin>> {
    vec!(
        Box::new(tokenize::tokenize {}),
        Box::new(body::body {}),
        Box::new(shell::shell {}),
        Box::new(hash::hash {}),
    )
}
