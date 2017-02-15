// @Author: ronan
// @Date:   22-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use pcsc_sys::*;

#[derive(Debug,Clone,Copy)]
///How to dispose (aka quit) the card.
pub enum CardDisposition {
    Leave,
    Reset,
    Unpower,
    Eject,
    Auto
}

impl CardDisposition {
    pub fn to_value(&self) -> DWORD {
        match self {
            &CardDisposition::Leave     => SCARD_LEAVE_CARD,
            &CardDisposition::Reset     => SCARD_RESET_CARD,
            &CardDisposition::Unpower   => SCARD_UNPOWER_CARD,
            &CardDisposition::Eject     => SCARD_EJECT_CARD,
            &CardDisposition::Auto      => SCARD_LEAVE_CARD
        }
    }
}
