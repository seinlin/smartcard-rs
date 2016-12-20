// @Author: ronan
// @Date:   20-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 20-12-2016



/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub const MAX_ATR_SIZE: ::std::os::raw::c_ulong = 33;
pub const SCARD_AUTOALLOCATE: ::std::os::raw::c_ulong = 0xFFFFFFFFFFFFFFFF;
pub const SCARD_SCOPE_USER: ::std::os::raw::c_ulong = 0;
pub const SCARD_SCOPE_TERMINAL: ::std::os::raw::c_ulong = 1;
pub const SCARD_SCOPE_SYSTEM: ::std::os::raw::c_ulong = 2;
pub const SCARD_PROTOCOL_UNDEFINED: ::std::os::raw::c_ulong = 0;
pub const SCARD_PROTOCOL_UNSET: ::std::os::raw::c_ulong = 0;
pub const SCARD_PROTOCOL_T0: ::std::os::raw::c_ulong = 1;
pub const SCARD_PROTOCOL_T1: ::std::os::raw::c_ulong = 2;
pub const SCARD_PROTOCOL_RAW: ::std::os::raw::c_ulong = 4;
pub const SCARD_PROTOCOL_T15: ::std::os::raw::c_ulong = 8;
pub const SCARD_PROTOCOL_ANY: ::std::os::raw::c_ulong = 3;
pub const SCARD_SHARE_EXCLUSIVE: ::std::os::raw::c_ulong = 1;
pub const SCARD_SHARE_SHARED: ::std::os::raw::c_ulong = 2;
pub const SCARD_SHARE_DIRECT: ::std::os::raw::c_ulong = 3;
pub const SCARD_LEAVE_CARD: ::std::os::raw::c_ulong = 0;
pub const SCARD_RESET_CARD: ::std::os::raw::c_ulong = 1;
pub const SCARD_UNPOWER_CARD: ::std::os::raw::c_ulong = 2;
pub const SCARD_EJECT_CARD: ::std::os::raw::c_ulong = 3;
pub const SCARD_UNKNOWN: ::std::os::raw::c_ulong = 1;
pub const SCARD_ABSENT: ::std::os::raw::c_ulong = 2;
pub const SCARD_PRESENT: ::std::os::raw::c_ulong = 4;
pub const SCARD_SWALLOWED: ::std::os::raw::c_ulong = 8;
pub const SCARD_POWERED: ::std::os::raw::c_ulong = 16;
pub const SCARD_NEGOTIABLE: ::std::os::raw::c_ulong = 32;
pub const SCARD_SPECIFIC: ::std::os::raw::c_ulong = 64;
pub const SCARD_STATE_UNAWARE: ::std::os::raw::c_ulong = 0;
pub const SCARD_STATE_IGNORE: ::std::os::raw::c_ulong = 1;
pub const SCARD_STATE_CHANGED: ::std::os::raw::c_ulong = 2;
pub const SCARD_STATE_UNKNOWN: ::std::os::raw::c_ulong = 4;
pub const SCARD_STATE_UNAVAILABLE: ::std::os::raw::c_ulong = 8;
pub const SCARD_STATE_EMPTY: ::std::os::raw::c_ulong = 16;
pub const SCARD_STATE_PRESENT: ::std::os::raw::c_ulong = 32;
pub const SCARD_STATE_ATRMATCH: ::std::os::raw::c_ulong = 64;
pub const SCARD_STATE_EXCLUSIVE: ::std::os::raw::c_ulong = 128;
pub const SCARD_STATE_INUSE: ::std::os::raw::c_ushort = 256;
pub const SCARD_STATE_MUTE: ::std::os::raw::c_ushort = 512;
pub const SCARD_STATE_UNPOWERED: ::std::os::raw::c_ushort = 1024;
pub const INFINITE: ::std::os::raw::c_uint = 4294967295;
pub const PCSCLITE_MAX_READERS_CONTEXTS: ::std::os::raw::c_ulong = 16;
pub const MAX_READERNAME: ::std::os::raw::c_ulong = 128;
pub const SCARD_ATR_LENGTH: ::std::os::raw::c_ulong = 33;
pub const MAX_BUFFER_SIZE: ::std::os::raw::c_ushort = 264;
pub const MAX_BUFFER_SIZE_EXTENDED: ::std::os::raw::c_uint = 65548;
pub type BYTE = ::std::os::raw::c_uchar;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type PUCHAR = *mut UCHAR;
pub type USHORT = ::std::os::raw::c_ushort;
pub type ULONG = ::std::os::raw::c_ulong;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type DWORD = ::std::os::raw::c_ulong;
pub type PDWORD = *mut DWORD;
pub type LONG = ::std::os::raw::c_long;
pub type LPCSTR = *const ::std::os::raw::c_char;
pub type LPCBYTE = *const BYTE;
pub type LPBYTE = *mut BYTE;
pub type LPDWORD = *mut DWORD;
pub type LPSTR = *mut ::std::os::raw::c_char;
pub type LPTSTR = LPSTR;
pub type LPCTSTR = LPCSTR;
pub type BOOL = ::std::os::raw::c_short;
pub type WORD = ::std::os::raw::c_ushort;
pub type PULONG = *mut ULONG;
pub type SCARDCONTEXT = LONG;
pub type PSCARDCONTEXT = *mut SCARDCONTEXT;
pub type LPSCARDCONTEXT = *mut SCARDCONTEXT;
pub type SCARDHANDLE = LONG;
pub type PSCARDHANDLE = *mut SCARDHANDLE;
pub type LPSCARDHANDLE = *mut SCARDHANDLE;
#[repr(C)]
#[derive(Copy)]
pub struct SCARD_READERSTATE {
    pub szReader: *const ::std::os::raw::c_char,
    pub pvUserData: *mut ::std::os::raw::c_void,
    pub dwCurrentState: DWORD,
    pub dwEventState: DWORD,
    pub cbAtr: DWORD,
    pub rgbAtr: [::std::os::raw::c_uchar; 33usize],
}
impl ::std::clone::Clone for SCARD_READERSTATE {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for SCARD_READERSTATE {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type LPSCARD_READERSTATE = *mut SCARD_READERSTATE;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: ::std::os::raw::c_ulong,
    pub cbPciLength: ::std::os::raw::c_ulong,
}
impl ::std::default::Default for SCARD_IO_REQUEST {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type PSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
pub type LPSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
pub type LPCSCARD_IO_REQUEST = *const SCARD_IO_REQUEST;
extern "C" {
    pub static g_rgSCardT0Pci: SCARD_IO_REQUEST;
    pub static g_rgSCardT1Pci: SCARD_IO_REQUEST;
    pub static g_rgSCardRawPci: SCARD_IO_REQUEST;
}

#[link(name="pcsclite")]
extern "C" {
    pub fn pcsc_stringify_error(arg1: LONG) -> *mut ::std::os::raw::c_char;
    pub fn SCardEstablishContext(dwScope: DWORD, pvReserved1: LPCVOID,
                                 pvReserved2: LPCVOID,
                                 phContext: LPSCARDCONTEXT) -> LONG;
    pub fn SCardReleaseContext(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardIsValidContext(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardConnect(hContext: SCARDCONTEXT, szReader: LPCSTR,
                        dwShareMode: DWORD, dwPreferredProtocols: DWORD,
                        phCard: LPSCARDHANDLE, pdwActiveProtocol: LPDWORD)
     -> LONG;
    pub fn SCardReconnect(hCard: SCARDHANDLE, dwShareMode: DWORD,
                          dwPreferredProtocols: DWORD,
                          dwInitialization: DWORD, pdwActiveProtocol: LPDWORD)
     -> LONG;
    pub fn SCardDisconnect(hCard: SCARDHANDLE, dwDisposition: DWORD) -> LONG;
    pub fn SCardBeginTransaction(hCard: SCARDHANDLE) -> LONG;
    pub fn SCardEndTransaction(hCard: SCARDHANDLE, dwDisposition: DWORD)
     -> LONG;
    pub fn SCardStatus(hCard: SCARDHANDLE, mszReaderName: LPSTR,
                       pcchReaderLen: LPDWORD, pdwState: LPDWORD,
                       pdwProtocol: LPDWORD, pbAtr: LPBYTE,
                       pcbAtrLen: LPDWORD) -> LONG;
    pub fn SCardGetStatusChange(hContext: SCARDCONTEXT, dwTimeout: DWORD,
                                rgReaderStates: LPSCARD_READERSTATE,
                                cReaders: DWORD) -> LONG;
    pub fn SCardControl(hCard: SCARDHANDLE, dwControlCode: DWORD,
                        pbSendBuffer: LPCVOID, cbSendLength: DWORD,
                        pbRecvBuffer: LPVOID, cbRecvLength: DWORD,
                        lpBytesReturned: LPDWORD) -> LONG;
    pub fn SCardTransmit(hCard: SCARDHANDLE,
                         pioSendPci: *const SCARD_IO_REQUEST,
                         pbSendBuffer: LPCBYTE, cbSendLength: DWORD,
                         pioRecvPci: *mut SCARD_IO_REQUEST,
                         pbRecvBuffer: LPBYTE, pcbRecvLength: LPDWORD)
     -> LONG;
    pub fn SCardListReaderGroups(hContext: SCARDCONTEXT, mszGroups: LPSTR,
                                 pcchGroups: LPDWORD) -> LONG;
    pub fn SCardListReaders(hContext: SCARDCONTEXT, mszGroups: LPCSTR,
                            mszReaders: LPSTR, pcchReaders: LPDWORD) -> LONG;
    pub fn SCardFreeMemory(hContext: SCARDCONTEXT, pvMem: LPCVOID) -> LONG;
    pub fn SCardCancel(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardGetAttrib(hCard: SCARDHANDLE, dwAttrId: DWORD, pbAttr: LPBYTE,
                          pcbAttrLen: LPDWORD) -> LONG;
    pub fn SCardSetAttrib(hCard: SCARDHANDLE, dwAttrId: DWORD,
                          pbAttr: LPCBYTE, cbAttrLen: DWORD) -> LONG;
}
