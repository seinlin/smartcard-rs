<!--
@Author: ronan
@Date:   22-12-2016
@Email:  ronan.lashermes@inria.fr
@Last modified by:   ronan
@Last modified time: 28-12-2016
-->

This is an early version library, everything can break at any moment.

# Smartcard-rs

This library enables communication between a PC and a smartcard (SC), using PC/SC drivers.

## How to use this library

### Add the crate

In your **Cargo.toml**:
```toml
[dependencies]
smartcard = "0.3"
```

In your *main.rs* or *lib.rs*:
```rust
extern crate smartcard;
```

### Example code

In the *main.rs*:
```rust
extern crate smartcard;

use smartcard::logic::Context;
use smartcard::parameters::{ShareMode, Protocol};
use smartcard::errors::*;

use std::rc::Rc;

fn run() -> Result<()> {
    //First we create the resource manager context. I think of it as 'the driver'.
    let context = Rc::new(try!(Context::establish_context_auto()));

    //The context allows to list all available card readers.
    let mut readers = try!(context.list_readers());

    println!("{} readers found:", readers.len());
    for r in readers.iter() {
        println!("- {}", r.get_name());
    }

    //Let's get the first reader.
    let reader = try!(readers.pop().ok_or(format!("no readers found")));

    //From the reader, we can connect to its smartcard this way.
    let card = try!(reader.connect_to(context.clone(), ShareMode::Auto, Protocol::Auto));
    //we clone the Rc<context> so that even if we
    //drop(context)
    //the context still exists while the card is alive

    //Now that we have a card available, we can send commands to it.
    //select app on my card
    let cmd_vec = vec![0x00, 0xA4, 0x04, 0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00];
    let answer = try!(card.send_raw_command(&cmd_vec, 256));//256 is the maximum size of the expected answer

    println!("Answer: {:?}", answer);//I get 0x90 0x00, perfect!
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => println!("An error occured: {}.", e.to_string())
    }
}
```

## Supported platform and dependencies

The **[pcsc-sys](https://github.com/bluetech/pcsc-rust)** crate is used to link with the drivers.
It should work on Linux, Windows and MacOS but I only tested it on Linux.

On ubuntu based systems:
```
sudo apt-get install pcscd libpcsclite1
```
Optionnally you can install the always useful pcsc-tools package:
```
sudo apt-get install pcsc-tools
```
With this tools you can detect your readers with
```
pcsc_scan
```

PRs welcome.

## Alternatives

You can check [pcsc-rust](https://github.com/bluetech/pcsc-rust) for another library with the same goal. I reused the **pcsc-sys** crate from this project for the lib bindings (starting from **smartcard-rs v 0.3.0** ).
**pcsc-rust** should have better performances than **smartcard-rs** at the cost of a slightly less ergonomic API (in my opinion).
Finally, in **pcsc-rust** the Card and Context are linked with lifetimes to prevent inappropriate Context dropping (instead of Rc in **smartcard-rs** case), which can cause self borrowing issues in some applications.
