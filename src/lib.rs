// @Author: ronan
// @Date:   20-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 21-12-2016

extern crate libc;

// #![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod scard;
pub mod safe;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}
