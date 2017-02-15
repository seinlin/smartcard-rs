// @Author: ronan
// @Date:   21-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use errors::*;
use pcsc_sys::*;

use std::ffi::CString;

///Parse an error code and transform it as a result
pub fn parse_error_code(code: LONG) -> Result<()> {
    //We could use pcsc_stringify_error but this method is not in the shared scard API

    match code {
        SCARD_S_SUCCESS                         => Ok(()),
        SCARD_F_INTERNAL_ERROR                  => bail!("internal error"),
        SCARD_E_CANCELLED                       => bail!("the action was cancelled by a request"),
        SCARD_E_INVALID_PARAMETER               => bail!("one or more of the supplied parameters can not be properly interpreted"),
        SCARD_E_INVALID_HANDLE                  => bail!("the supplied handle is not valid"),
        SCARD_E_INVALID_TARGET                  => bail!("registry startup information is missing or not valid"),
        SCARD_E_NO_MEMORY                       => bail!("not enough memory available to complete this command"),
        SCARD_F_WAITED_TOO_LONG                 => bail!("an internal consistency timer has expired"),
        SCARD_E_INSUFFICIENT_BUFFER             => bail!("the data buffer for returned data is too small for the returned data"),
        SCARD_E_UNKNOWN_READER                  => bail!("the specified reader name is not recognized"),
        SCARD_E_TIMEOUT                         => bail!("the user-specified time-out value has expired"),
        SCARD_E_SHARING_VIOLATION               => bail!("the smart card cannot be accessed because of other outstanding connections"),
        SCARD_E_NO_SMARTCARD                    => bail!("the operation requires a smart card, but no smart card is currently in the device"),
        SCARD_E_UNKNOWN_CARD                    => bail!("the specified smart card name is not recognized"),
        SCARD_E_CANT_DISPOSE                    => bail!("the system could not dispose of the media in the requested manner"),
        SCARD_E_PROTO_MISMATCH                  => bail!("the requested protocols are incompatible with the protocol currently in use with the card"),
        SCARD_E_NOT_READY                       => bail!("the reader or card is not ready to accept commands"),
        SCARD_E_NO_READERS_AVAILABLE            => bail!("group contains no readers"),
        SCARD_E_READER_UNAVAILABLE              => bail!("specified reader is not currently available for use"),
        SCARD_E_INVALID_VALUE                   => bail!("one or more of the supplied parameter values could not be properly interpreted"),
        SCARD_E_SYSTEM_CANCELLED                => bail!("the action was canceled by the system, presumably to log off or shut down"),
        SCARD_F_COMM_ERROR                      => bail!("an internal communications error has been detected"),
        SCARD_F_UNKNOWN_ERROR                   => bail!("an internal error has been detected, but the source is unknown"),
        SCARD_E_INVALID_ATR                     => bail!("an ATR string obtained from the registry is not a valid ATR string"),
        SCARD_E_NOT_TRANSACTED                  => bail!("an attempt was made to end a nonexistent transaction"),
        SCARD_P_SHUTDOWN                        => bail!("the operation has been aborted to allow the server application to exit"),
        SCARD_E_PCI_TOO_SMALL                   => bail!("the PCI receive buffer was too small"),
        SCARD_E_READER_UNSUPPORTED              => bail!("the reader driver does not meet minimal requirements for support"),
        SCARD_E_DUPLICATE_READER                => bail!("the reader driver did not produce a unique reader name"),
        SCARD_E_CARD_UNSUPPORTED                => bail!("the smart card does not meet minimal requirements for support"),
        SCARD_E_NO_SERVICE                      => bail!("the smart card resource manager is not running"),
        SCARD_E_SERVICE_STOPPED                 => bail!("the smart card resource manager has shut down"),
        SCARD_E_UNEXPECTED                      => bail!("an unexpected card error has occurred"),
        // SCARD_E_UNSUPPORTED_FEATURE             => bail!("this smart card does not support the requested feature"),
        SCARD_E_ICC_INSTALLATION                => bail!("no primary provider can be found for the smart card"),
        SCARD_E_ICC_CREATEORDER                 => bail!("the requested order of object creation is not supported"),
        SCARD_E_DIR_NOT_FOUND                   => bail!("the specified directory does not exist in the smart card"),
        SCARD_E_FILE_NOT_FOUND                  => bail!("the specified file does not exist in the smart card"),
        SCARD_E_NO_DIR                          => bail!("the supplied path does not represent a smart card directory"),
        SCARD_E_NO_FILE                         => bail!("the supplied path does not represent a smart card file"),
        SCARD_E_NO_ACCESS                       => bail!("access is denied to the file"),
        SCARD_E_WRITE_TOO_MANY                  => bail!("an attempt was made to write more data than would fit in the target object"),
        SCARD_E_BAD_SEEK                        => bail!("an error occurred in setting the smart card file object pointer."),
        SCARD_E_INVALID_CHV                     => bail!("the supplied PIN is incorrect"),
        SCARD_E_UNKNOWN_RES_MNG                 => bail!("an unrecognized error code was returned"),
        SCARD_E_NO_SUCH_CERTIFICATE             => bail!("the requested certificate does not exist"),
        SCARD_E_CERTIFICATE_UNAVAILABLE         => bail!("the requested certificate could not be obtained"),
        SCARD_E_COMM_DATA_LOST                  => bail!("a communications error with the smart card has been detected"),
        SCARD_E_NO_KEY_CONTAINER                => bail!("the requested key container does not exist on the smart card"),
        SCARD_E_SERVER_TOO_BUSY                 => bail!("the smart card resource manager is too busy to complete this operation"),
        SCARD_W_UNSUPPORTED_CARD                => bail!("the reader cannot communicate with the card, due to ATR string configuration conflicts"),
        SCARD_W_UNRESPONSIVE_CARD               => bail!("the smart card is not responding to a reset"),
        SCARD_W_UNPOWERED_CARD                  => bail!("power has been removed from the smart card, so that further communication is not possible"),
        SCARD_W_RESET_CARD                      => bail!("the smart card was reset"),
        SCARD_W_REMOVED_CARD                    => bail!("the smart card has been removed, so further communication is not possible"),
        SCARD_W_SECURITY_VIOLATION              => bail!("access was denied because of a security violation"),
        SCARD_W_WRONG_CHV                       => bail!("the card cannot be accessed because the wrong PIN was presented"),
        SCARD_W_CHV_BLOCKED                     => bail!("the card cannot be accessed because the maximum number of PIN entry attempts has been reached"),
        SCARD_W_EOF                             => bail!("the end of the smart card file has been reached"),
        SCARD_W_CANCELLED_BY_USER               => bail!("the action was canceled by the user"),
        SCARD_W_CARD_NOT_AUTHENTICATED          => bail!("no PIN was presented to the smart card"),

        _                                       => bail!("unknown error")
    }
}

///Parse a multi-cstring: several strings are separated by a /00, the end of the multi-cstring is marked by a double /00 /00.
pub unsafe fn parse_multi_cstring(cstr: CString, max_size: u64) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut current_str = String::new();
    let mut consecutive_null = 0usize;

    let ptr = cstr.as_ptr();

    //Explore only up to max_size chars
    for i in 0..(max_size as isize) {
        let c = *ptr.offset(i) as u8 as char;
        if c == '\0' {
            consecutive_null += 1;

            if consecutive_null == 2 {//end of multi-string
                break;
            }

            //add string to result and clear current
            result.push(current_str.clone());
            current_str.clear();
        }
        else {
            consecutive_null = 0;
            current_str.push(c);
        }
    }

    //add last string if not empty
    if current_str.len() != 0 {
        result.push(current_str);
    }

    result
}
