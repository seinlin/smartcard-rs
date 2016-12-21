// @Author: ronan
// @Date:   21-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 21-12-2016

use std::ptr;

use safe::utils::parse_error_code;
use errors::*;
use scard::winscard::{SCARDCONTEXT, DWORD, SCardEstablishContext, SCardReleaseContext, SCARD_SCOPE_SYSTEM, SCARD_SCOPE_USER};

///The resource manager context is a representation of the state of the driver.
#[derive(Debug)]
pub struct Context {
    handle: SCARDCONTEXT
}

impl Context {
    fn establish_context(scope: DWORD) -> Result<Context> {
        let mut h_context: SCARDCONTEXT = SCARDCONTEXT::default();

        try!(//return early if error
            parse_error_code(//convert error code to result
                unsafe { SCardEstablishContext(scope, ptr::null(), ptr::null(), &mut h_context) }));//establish context

        //Return new context
        Ok(Context { handle: h_context })
    }

    ///Create a context in the system scope
    pub fn establish_context_system() -> Result<Context> {
        Context::establish_context(SCARD_SCOPE_SYSTEM)
    }

    ///Create a context in the user scope
    pub fn establish_context_user() -> Result<Context> {
        Context::establish_context(SCARD_SCOPE_USER)
    }

    ///Create a context
    ///For when you dont want to choose the scope (alias to establish_context_system)
    pub fn establish_context_auto() -> Result<Context> {
        Context::establish_context_system()
    }

    pub fn list_readers(&self) -> Result<Vec<Reader>> {
        let mut reader_str = String::new();
        unsafe {

            let mut str_size = 256u64;
            let mut readers_ptr = try!(CString::new(Vec::with_capacity(str_size as usize)));
            let mut str_ptr = readers_ptr.into_raw();
            try!(
                parse_error_code(
                    SCardListReaders(h_context, ptr::null(), str_ptr, &mut str_size)));
            readers_ptr = CString::from_raw(str_ptr);

            reader_str = try!(String::from_utf8(readers_cstr.to_bytes().to_vec()));
        }

        //split on NULL \x00
        let readers_names = reader_str.split("\0");

        //TODO

    }
}

///Free the context at the end
impl Drop for Context {
    fn drop(&mut self) {
        //Release the context (do not deal with the error code since we cant do anything at this step)
        unsafe { SCardReleaseContext(self.handle) };
    }
}


#[test]
///The drivers should be available to pass
fn test_context_creation() {
    let context = Context::establish_context_auto();
    assert!(context.is_ok());
}
