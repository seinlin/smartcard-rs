// @Author: ronan
// @Date:   23-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 23-12-2016

use scard::winscard::{DWORD,
    SCARD_LEAVE_CARD,
    SCARD_RESET_CARD,
    SCARD_UNPOWER_CARD};

#[derive(Debug,Clone,Copy)]
///Type of initialization to reconnect the card
pub enum InitializationType {
    Leave,
    Reset,
    Unpower,
    Auto
}

impl InitializationType {
    pub fn to_value(&self) -> DWORD {
        match self {
            &InitializationType::Leave     => SCARD_LEAVE_CARD,
            &InitializationType::Reset     => SCARD_RESET_CARD,
            &InitializationType::Unpower   => SCARD_UNPOWER_CARD,
            &InitializationType::Auto      => SCARD_LEAVE_CARD
        }
    }
}
