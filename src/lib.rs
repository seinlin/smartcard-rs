// @Author: ronan
// @Date:   20-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 20-12-2016

extern crate libc;

#[macro_use]
extern crate error_chain;

pub mod scard;


#[test]
fn test_scard() {
    use std::ptr;
    use std::ffi::CString;
    // use scard::winscard::{SCARDCONTEXT, SCardEstablishContext, SCardReleaseContext, SCARD_SCOPE_SYSTEM};
    use scard::winscard::*;


    let mut h_context: SCARDCONTEXT = SCARDCONTEXT::default();
    let mut dwReaders = SCARD_AUTOALLOCATE;
    let mut mszReaders = CString::new("data data data").unwrap();

    unsafe {
        SCardEstablishContext(SCARD_SCOPE_SYSTEM, ptr::null(), ptr::null(), &mut h_context);



        SCardListReaders(h_context, ptr::null(), mszReaders.as_ptr() as LPSTR, &mut dwReaders);
        println!("Readers: {:?}", mszReaders);

        SCardReleaseContext(h_context);
    }


}
