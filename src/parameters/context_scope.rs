// @Author: ronan
// @Date:   22-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use pcsc_sys::*;

#[derive(Debug,Clone,Copy)]
///The scope of the resource manager context
pub enum ContextScope {
    System,
    User,
    Auto
}

impl ContextScope {
    pub fn to_value(&self) -> DWORD {
        match self {
            &ContextScope::System => SCARD_SCOPE_SYSTEM,
            &ContextScope::User => SCARD_SCOPE_USER,
            &ContextScope::Auto => SCARD_SCOPE_SYSTEM,
        }
    }
}
