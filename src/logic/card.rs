// @Author: ronan
// @Date:   22-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 23-12-2016

use errors::*;

use pcsc_sys::*;

use parameters::{Protocol, ShareMode, CardDisposition, InitializationType};
use logic::utils::parse_error_code;
use logic::{Context, Reader};

use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

#[allow(dead_code)]
///This struct represents a smartcard.
// #[derive(Debug)]
pub struct Card {
    handle:         SCARDHANDLE,
    context:        Arc<Context>,//get a reference counter on the context: prevent context dropping while this card is alive
    protocol:       Protocol,
    share:          ShareMode,
    to_disconnect:     bool
}

impl Card {
    /// Connect to a smartcard on the specified reader
    /// # Arguments
    /// * `context` - The resource manager context.
    /// * `reader` - The reader that contains the smartcard you want to connect to.
    /// * `share_mode` - How do you want to share the access to the smartcard.
    /// * `preferred_protocol` - What protocol do you want to use to connect to the smartcard.
    pub fn connect_to(context: Arc<Context>, reader: &Reader, share_mode: ShareMode, preferred_protocol: Protocol) -> Result<Card> {
        info!("Trying to connect to reader {}.", reader.get_name());
        let mut card_handle: SCARDHANDLE = SCARDHANDLE::default();//allocate to receive card handle value
        let mut protocol_choice: DWORD = DWORD::default();//allocate to receive chosen protocol
        unsafe {
            let reader_cstr = CString::new(reader.get_name()) .chain_err(|| "failed to convert to CString")?;
            parse_error_code(
                SCardConnect(context.get_handle(), reader_cstr.as_ptr(), share_mode.to_value(), preferred_protocol.to_value(), &mut card_handle, &mut protocol_choice)
            )?;

        }
        let chosen_protocol = Protocol::from(protocol_choice);

        info!("Connection to reader {} achieved.", reader.get_name());
        Ok(Card { handle: card_handle, context: context, protocol: chosen_protocol, share: share_mode, to_disconnect: true})
    }

    /// Get a pointer to the static PCI from the specified protocol
    pub fn get_pci(&self) -> Result<&SCARD_IO_REQUEST> {
        let pci = unsafe { match self.protocol {
            Protocol::T0    => &g_rgSCardT0Pci,
            Protocol::T1    => &g_rgSCardT1Pci,
            Protocol::Raw   => &g_rgSCardRawPci,
            _               => bail!("chosen protocol ({}) is not implemented")
        }};
        Ok(pci)
    }

    /// Reconnect to the smartcard
    /// # Arguments
    /// * `init_type` - How to reset the card.
    pub fn reconnect(&mut self, init_type: InitializationType) -> Result<()> {
        let mut protocol_choice: DWORD = DWORD::default();//allocate to receive chosen protocol
        let result = unsafe {
                parse_error_code(
                    SCardReconnect(self.handle, self.share.to_value(), self.protocol.to_value(), init_type.to_value(), &mut protocol_choice))
        };

        //if we fail to reconnect, disconnect in drop becomes useless
        if result.is_err() {
            self.to_disconnect = false;
            return result;
        }
        else {//reconnect is a success, so disconnect is necessary
            self.to_disconnect = true;
        }

        let chosen_protocol = Protocol::from(protocol_choice);
        self.protocol = chosen_protocol;

        info!("Card reconnection achieved.");
        Ok(())
    }

    /// Send a raw command to the smartcard.
    /// # Arguments
    /// * `cmd` - The cmd you want to send.
    /// * `max_answer_size` - The maximum size of the expected answer.
    pub fn send_raw_command(&self, cmd: &[u8], max_answer_size: usize) -> Result<Vec<u8>> {
        debug!("Sending command {:?} expecting {} bytes in answer at most.", cmd, max_answer_size);
        let mut rx_vec = vec![0;max_answer_size];
        let mut rx_size = max_answer_size as u32;
        unsafe {
            let cmd_buf_ptr = cmd.as_ptr();
            let rx_buf_ptr = rx_vec.as_mut_ptr();
            parse_error_code(
                SCardTransmit(self.handle, self.get_pci()?, cmd_buf_ptr, cmd.len() as u32, ptr::null_mut(), rx_buf_ptr, &mut rx_size)
            )?;
        }
        rx_vec.truncate(rx_size as usize);
        Ok(rx_vec)
    }
}

impl Drop for Card {
    ///Disconnect the card at the end
    fn drop(&mut self) {
        //There are cases when we dont want to disconnect. Ex: failed reset so card is not connected
        if self.to_disconnect {
            unsafe { SCardDisconnect(self.handle, CardDisposition::Auto.to_value()); } //cannot deal with fail here
        }
    }
}

//Test requires a reader and *my* smartcard to be plugged.
// #[test]
// fn test_card() {
//     let context = Context::establish_context_auto().unwrap();
//     let mut readers = context.list_readers().unwrap();
//
//     println!("{} readers found:", readers.len());
//     for r in readers.iter() {
//         println!("- {}", r.get_name());
//     }
//
//     let reader = match readers.pop() {
//         Some(r) => r,
//         None => panic!("No reader found!")
//     };
//
//     let card = reader.connect_to(&context, ShareMode::Auto, Protocol::Auto).unwrap();
//     let cmd_vec = vec![0x00, 0xA4, 0x04, 0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00];//select app on my card
//     let answer = card.send_raw_command(&cmd_vec, 256).unwrap();
//
//     println!("Answer: {:?}", answer);
// }
