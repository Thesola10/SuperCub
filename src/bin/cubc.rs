//! GCC-compatible wrapper for Super Cub.
//! Redirects most options as-is to the compiler. Calls $CC if set, else
//! command 'cc' in PATH.

use supercub::builtins::{BUILTINS, Consumer};

fn main() {
    println!("builtin: {}", BUILTINS[0].name);
    match BUILTINS[0].consumer {
        Consumer::Macro(_) => println!("... is a macro"),
        Consumer::Decorator(_) => println!("... is a decorator")
    }
    println!("please bear with me!");
}
