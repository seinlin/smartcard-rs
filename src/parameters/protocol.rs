// @Author: ronan
// @Date:   22-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use scard::winscard::{SCARD_PROTOCOL_UNDEFINED,
    SCARD_PROTOCOL_T0,
    SCARD_PROTOCOL_T1,
    SCARD_PROTOCOL_RAW,
    SCARD_PROTOCOL_T15,
    SCARD_PROTOCOL_ANY,
    DWORD};

use errors::*;

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

    pub fn from_value(val: DWORD) -> Result<Protocol> {
        match val {
            SCARD_PROTOCOL_T0           => Ok(Protocol::T0),
            SCARD_PROTOCOL_T1           => Ok(Protocol::T1),
            SCARD_PROTOCOL_UNDEFINED    => Ok(Protocol::Undefined),
            SCARD_PROTOCOL_ANY          => Ok(Protocol::Any),
            SCARD_PROTOCOL_RAW          => Ok(Protocol::Raw),
            SCARD_PROTOCOL_T15          => Ok(Protocol::T15),
            _                           => bail!("Impossible to decode protocol code ({})", val),
        }
    }
}
