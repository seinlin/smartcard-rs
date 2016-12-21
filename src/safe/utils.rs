// @Author: ronan
// @Date:   21-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 21-12-2016

use errors::*;
use scard::winscard::*;

pub fn parse_error_code(code: LONG) -> Result<()> {
    match code {
        SCARD_S_SUCCESS => Ok(()),
        SCARD_F_INTERNAL_ERROR => bail!("internal error"),
        SCARD_E_NO_READERS_AVAILABLE => bail!("group contains no readers"),
        SCARD_E_READER_UNAVAILABLE => bail!("specified reader is not currently available for use"),
        _ => bail!("unknown error")
    }
}
