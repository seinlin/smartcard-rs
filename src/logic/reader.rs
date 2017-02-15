// @Author: ronan
// @Date:   21-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 22-12-2016

use errors::*;

use logic::{Context, Card};
use parameters::{ShareMode, Protocol};

use std::rc::Rc;

#[derive(Debug,Clone)]
///The card reader is what is really plugged into the computer.
pub struct Reader {
    name: String,
}

impl Reader {

    ///Constructor for a reader.
    ///Prefer to select a reader from the list created by the context.
    pub fn new(reader_name: String) -> Reader {
        Reader { name: reader_name }
    }

    ///Get the name of this reader.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Connect to a smartcard on this reader
    /// # Arguments
    /// * `context` - The resource manager context.
    /// * `share_mode` - How do you want to share the access to the smartcard.
    /// * `preferred_protocol` - What protocol do you want to use to connect to the smartcard.
    pub fn connect_to(&self, context: Rc<Context>, share_mode: ShareMode, preferred_protocol: Protocol) -> Result<Card> {
        Card::connect_to(context, self, share_mode, preferred_protocol)
    }
}

//Test requires a reader and a smartcard to be plugged.
// #[test]
// fn test_reader() {
//     let context = Context::establish_context_auto().unwrap();
//     let mut readers = context.list_readers().unwrap();
//     let reader = match readers.pop() {
//         Some(r) => r,
//         None => panic!("No reader found!")
//     };
//
//     let _ = reader.connect_to(&context, ShareMode::Auto, Protocol::Auto).unwrap();
// }
