//! Set of built-in macros in Super Cub.

#![macro_escape]

use crate::parser::ast;

use crate::realize::{Realizable, Env};

pub mod tokenize;
pub mod body;
pub mod shell;
pub mod hash;

/// Gathered builtins defined below
#[distributed_slice]
pub static BUILTINS: [&'static Builtin];

type MacroConsumer = fn(args: Vec<Box<str>>, env: Vec<Env>) -> String;
type DecoratorConsumer = fn(args: Vec<Box<str>>, target: ast::CChunk) -> String;

pub enum Consumer {
    Macro(MacroConsumer),
    Decorator(DecoratorConsumer)
}

pub struct Builtin {
    pub name: &'static str,
    pub consumer: Consumer
}

/// Specialization macro for converting a function into an implementation of Macro
#[macro_export]
macro_rules! builtin_macro {(
    $(#[$fn_meta:meta])*
    $fn_vis:vis
    fn $NAME:ident ($($param:ident : $type:ty),*) $body:block
) => (

    with_eager_expansions! {
        #[distributed_slice(BUILTINS)]
        static #{concat_idents!(BUILTIN_MACRO_, $NAME)}: &'static Builtin =
            &Builtin {
                name: stringify!($NAME),
                consumer: Consumer::Macro(|$($param : $type),*| -> String
                    $body )
            };
    }
)}

/// Specialization macro for converting a function into an implementation of Decorator
#[macro_export]
macro_rules! builtin_decor {(
    $(#[$fn_meta:meta])*
    $fn_vis:vis
    fn $NAME:ident ($($param:ident : $type:ty),*) $body:block
) => (
    with_eager_expansions! {
        #[distributed_slice(BUILTINS)]
        static #{concat_idents!(BUILTIN_DECOR_, $NAME)}: &'static Builtin =
            &Builtin {
                name: stringify!($NAME),
                consumer: Consumer::Decorator(|$($param : $type),*| -> String
                    $body )
            };
    }
)}

pub(crate) use builtin_macro;
pub(crate) use builtin_decor;

