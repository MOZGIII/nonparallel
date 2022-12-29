//! A procedural macro for Rust that allows you to ensure that functions (e.g.
//! unit tests) are not running at the same time.
//!
//! This is achieved by executing the provided expression (a mutex lock,
//! for instance) the beginning of the annotated function and holding on to
//! the returned value (for instance, the lock guard), such that the lock is
//! released when the function returns (or panics).
//!
//! This is useful to inject locks into functions that altered by other macros,
//! like `tokio::test`, and where you can't inject your lock expression by other
//! means.
//!
//! Macros are applied from top to bottom, when using with `tokio:test` and
//! similar macros, be sure to insert the `nonparallel` call **last**.
//! This will wrap the entire runtime with the lock.
//! If you insert `nonparallel` above the `tokio::test`, the lock will be
//! executed inside the runtime, which means the runtime will shutdown after
//! the lock is released, and may lead to things you meant to be under mutex to
//! run outside of mutex until the runtime actually shuts down.
//!
//! ## Usage
//!
//! ```rust
//! use tokio::sync::Mutex;
//! use nonparallelex::nonparallel;
//!
//! // Create two locks
//! static MUT_A: Mutex<()> = Mutex::const_new(());
//! static MUT_B: Mutex<()> = Mutex::const_new(());
//!
//! // Mutually exclude parallel runs of functions using those two locks.
//!
//! #[tokio::test]
//! #[nonparallel(MUT_A.blocking_lock())]
//! async fn function_a1() {
//!     // This will not run in parallel to function_a2.
//! }
//!
//! #[tokio::test]
//! #[nonparallel(MUT_A.blocking_lock())]
//! async fn function_a2() {
//!     // This will not run in parallel to function_a1.
//! }
//!
//! #[tokio::test]
//! #[nonparallel(MUT_B.blocking_lock())]
//! async fn function_b() {
//!     // This may run in parallel to function_a*.
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse, parse_macro_input, Expr, ItemFn, Stmt};

struct Nonparallel {
    expr: Expr,
}

impl Parse for Nonparallel {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let expr = input.parse::<Expr>()?;
        Ok(Nonparallel { expr })
    }
}

#[proc_macro_attribute]
pub fn nonparallel(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse macro attributes
    let Nonparallel { expr } = parse_macro_input!(attr);

    // Parse function
    let mut function: ItemFn = parse(item).expect("Could not parse ItemFn");

    // Insert locking code
    let quoted = quote! { let guard = #expr; };
    let stmt: Stmt = parse(quoted.into()).expect("Could not parse quoted statement");
    function.block.stmts.insert(0, stmt);

    // Generate token stream
    TokenStream::from(function.to_token_stream())
}
