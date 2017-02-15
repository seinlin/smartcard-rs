// @Author: ronan
// @Date:   21-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 23-12-2016

use std::ptr;
use std::ffi::CString;

use logic::utils::{parse_error_code, parse_multi_cstring};
use logic::reader::Reader;
use parameters::context_scope::ContextScope;

use errors::*;
use pcsc_sys::*;

///The resource manager context is a representation of the state of the driver.
#[derive(Debug)]
pub struct Context {
    handle: SCARDCONTEXT
}

impl Context {
    ///Create a context and define the scope of this context
    pub fn establish_context(scope: ContextScope) -> Result<Context> {
        let mut h_context: SCARDCONTEXT = SCARDCONTEXT::default();

        try!(//return early if error
            parse_error_code(//convert error code to result
                unsafe { SCardEstablishContext(scope.to_value(), ptr::null(), ptr::null(), &mut h_context) }));//establish context

        //Return new context
        debug!("Context created with scope {:?}.", scope);
        Ok(Context { handle: h_context })
    }

    ///Create a context
    ///For when you dont want to choose the scope (alias to establish_context system)
    pub fn establish_context_auto() -> Result<Context> {
        Context::establish_context(ContextScope::Auto)
    }

    ///Get the handle for this context
    pub fn get_handle(&self) -> SCARDCONTEXT {
        self.handle
    }

    ///Verify if a context is valid
    pub fn is_valid(&self) -> Result<bool> {
        let r_code = unsafe { SCardIsValidContext(self.handle) };
        match r_code {
            SCARD_E_INVALID_HANDLE  => Ok(false),
            rc                      => parse_error_code(rc).and(Ok(true))
        }
    }

    ///List all available readers
    pub fn list_readers(&self) -> Result<Vec<Reader>> {
        let readers_names = unsafe {
            //1# determine the required buffer len
            let mut buf_size = 0u64;
            try!(
                parse_error_code(
                    SCardListReaders(self.handle, ptr::null(), ptr::null_mut(), &mut buf_size)));

            //2# allocate the buffer
            let empty_buf = vec![0u8;buf_size as usize];
            let mut readers_ptr = CString::from_vec_unchecked(empty_buf);
            let str_ptr = readers_ptr.into_raw();

            //3# fill the buffer
            try!(
                parse_error_code(
                    SCardListReaders(self.handle, ptr::null(), str_ptr, &mut buf_size)));
            readers_ptr = CString::from_raw(str_ptr);

            //4# parse the buffer
            parse_multi_cstring(readers_ptr, buf_size)
        };

        //map reader names to reader struct
        let readers: Vec<Reader> = readers_names.into_iter().map(|name|Reader::new(name.to_string())).collect();

        info!("Available readers:");
        for r in readers.iter() {
            info!("- {}", r.get_name());
        }

        Ok(readers)
    }
}


impl Drop for Context {
    ///Free the context at the end
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
    assert!(context.unwrap().is_valid().unwrap());
}


#[test]
///The driver should be available to pass
fn test_list_readers() {
    let context = Context::establish_context_auto().unwrap();
    let _ = context.list_readers().unwrap();
}
