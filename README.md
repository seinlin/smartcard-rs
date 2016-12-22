<!--
@Author: ronan
@Date:   22-12-2016
@Email:  ronan.lashermes@inria.fr
@Last modified by:   ronan
@Last modified time: 22-12-2016
-->

This is a early version library, everything can break at any moment.

# Smartcard-rs

This library is a wrapper for the PC/SC driver. It enables communication between a PC and a smartcard (SC).

## Changelog

- 0.1.0: Initial version: minimal API allowing to send raw command to a smartcard.

## Example code


```rust
//First we create the resource manager context. I think of it as 'the driver'.
let context = try!(Context::establish_context_auto());

//The context allows to list all available card readers.
let mut readers = try!(context.list_readers());

println!("{} readers found:", readers.len());
for r in readers.iter() {
    println!("- {}", r.get_name());
}

//Let's get the first reader.
let reader = try!(readers.pop().ok_or(format!("no readers found")));

///From the reader, we can connect to its smartcard this way.
let card = try!(reader.connect_to(&context, ShareMode::Auto, Protocol::Auto));

//Now that we have a card available, we can send commands to it.
//select app on my card
let cmd_vec = vec![0x00, 0xA4, 0x04, 0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00];
let answer = try!(card.send_raw_command(&cmd_vec, 256));

println!("Answer: {:?}", answer);//I get 0x90 0x00, perfect!
```


## Supported platform and dependencies

Linux only for now. The library pcsclite and the pcscd deamon are required.

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

Windows and MacOS are not supported (yet?).
PRs welcome.
