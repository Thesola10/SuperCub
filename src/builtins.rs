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

type MacroConsumer = fn(args: Vec<&str>, env: Vec<Env>) -> Vec<Box<dyn Realizable>>;
type DecoratorConsumer = fn(target: ast::CChunk) -> Vec<Box<dyn Realizable>>;

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
                consumer: Consumer::Macro(|$($param : $type),*| -> Vec<Box<dyn Realizable>>
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
                consumer: Consumer::Decorator(|$($param : $type),*| -> Vec<Box<dyn Realizable>>
                    $body )
            };
    }
)}

pub(crate) use builtin_macro;
pub(crate) use builtin_decor;

