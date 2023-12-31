// @Author: ronan
// @Date:   22-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use pcsc_sys::*;

#[derive(Debug,Clone,Copy)]
///The communication protocol used with the smartcard.
pub enum Protocol {
    T0,
    T1,
    Undefined,
    ///Both T0 and T1
    Any,
    Raw,
    T15,
    Auto
}

impl Protocol {
    pub fn to_value(&self) -> DWORD {
        match self {
            &Protocol::T0           => SCARD_PROTOCOL_T0,
            &Protocol::T1           => SCARD_PROTOCOL_T1,
            &Protocol::Undefined    => SCARD_PROTOCOL_UNDEFINED,
            &Protocol::Any          => SCARD_PROTOCOL_ANY,
            &Protocol::Raw          => SCARD_PROTOCOL_RAW,
            &Protocol::T15          => SCARD_PROTOCOL_T15,
            &Protocol::Auto         => SCARD_PROTOCOL_ANY,
        }
    }
}

impl From<DWORD> for Protocol {
    fn from (val: DWORD) -> Protocol {
        match val {
            SCARD_PROTOCOL_T0           => Protocol::T0,
            SCARD_PROTOCOL_T1           => Protocol::T1,
            SCARD_PROTOCOL_UNDEFINED    => Protocol::Undefined,
            SCARD_PROTOCOL_ANY          => Protocol::Any,
            SCARD_PROTOCOL_RAW          => Protocol::Raw,
            SCARD_PROTOCOL_T15          => Protocol::T15,
            _                           => Protocol::Undefined
        }
    }
}
