// @Author: ronan
// @Date:   22-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use pcsc_sys::*;

#[derive(Debug,Clone,Copy)]
///How to share the control of the smartcard.
pub enum ShareMode {
    Shared,
    Exclusive,
    Direct,
    Auto
}

impl ShareMode {
    pub fn to_value(&self) -> DWORD {
        match self {
            &ShareMode::Shared          => SCARD_SHARE_SHARED,
            &ShareMode::Exclusive       => SCARD_SHARE_EXCLUSIVE,
            &ShareMode::Direct          => SCARD_SHARE_DIRECT,
            &ShareMode::Auto            => SCARD_SHARE_SHARED
        }
    }
}
