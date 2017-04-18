// @Author: ronan
// @Date:   20-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 23-12-2016

// #![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

extern crate pcsc_sys;

pub mod logic;
pub mod parameters;


// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

#[test]
fn test_readme() {
    use logic::Context;
    use parameters::{ShareMode, Protocol};

    use std::sync::Arc;

    //First we create the resource manager context. I think of it as 'the driver'.
    let context = Arc::new(Context::establish_context_auto().unwrap());

    //The context allows to list all available card readers.
    let mut readers = context.list_readers().unwrap();

    println!("{} readers found:", readers.len());
    for r in readers.iter() {
        println!("- {}", r.get_name());
    }

    //Let's get the first reader.
    let reader = readers.pop().ok_or(format!("no readers found")).unwrap();

    //From the reader, we can connect to its smartcard this way.
    let card = reader.connect_to(context, ShareMode::Auto, Protocol::Auto).unwrap();
    //we use an Arc<Context> so that even if we
    //drop(context)
    //the context still exists while the card is alive

    //Now that we have a card available, we can send commands to it.
    //select app on my card
    let cmd_vec = vec![0x00, 0xA4, 0x04, 0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00];
    let answer = card.send_raw_command(&cmd_vec, 256).unwrap();//256 is the maximum size of the expected answer

    println!("Answer: {:?}", answer);//I get 0x90 0x00, perfect!

}
