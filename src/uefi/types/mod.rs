mod status;
pub use status::Status;

mod memory_range;
pub use memory_range::MemoryRange;

mod guid;
pub use guid::GUID;

use core::ffi::c_void;

/// Logical Boolean. 1-byte value containing a 0 for FALSE or a 1 for TRUE. 
/// Other values are undefined.
pub type Boolean = bool;

/// Signed value of native width. (4 bytes on supported 32-bit processor 
/// instructions, 8 bytes on supported 64-bit processor instructions, 16 bytes 
/// on supported 128-bit processor instructions)
pub type IntN = isize;

/// Unsigned value of native width. (4 bytes on supported 32-bit processor 
/// instructions, 8 bytes on supported 64-bit processor instructions, 16 bytes 
/// on supported 128-bit processor instructions)
pub type UIntN = usize;

/// 1-byte signed value.
pub type Int8 = i8;

/// 1-byte unsigned value.
pub type UInt8 = u8;

/// 2-byte signed value.
pub type Int16 = i16;

/// 2-byte unsigned value.
pub type UInt16 = u16;

/// 4-byte signed value.
pub type Int32 = i32;

/// 4-byte unsigned value.
pub type UInt32 = u32;

/// 8-byte signed value.
pub type Int64 = i64;

/// 8-byte unsigned value.
pub type UInt64 = u64;

/// 16-byte signed value.
pub type Int128 = i128;

/// 16-byte unsigned value.
pub type UInt128 = u128;

/// 1-byte character. Unless otherwise specified, all 1-byte or ASCII characters
/// and strings are stored in 8-bit ASCII encoding format, using the ISO-Latin-1
/// character set.
pub type Char8 = u8;

/// 2-byte Character. Unless otherwise specified all characters and strings are
/// stored in the UCS-2 encoding format as defined by Unicode 2.1 and ISO/IEC
/// 10646 standards.
pub type Char16 = u16;

/// Undeclared type.
pub type Void = c_void;

/// A collection of related interfaces.
pub type Handle = *const Void;

/// Handle to an event structure. 
pub type Event = *const Void;

/// Logical block address. 
pub type LBA = UInt64; 

/// Task priority level. 
pub type TPL = UIntN;

/// 32-byte buffer containing a network Media Access Control address.
pub type MACAddress = crate::MACAddress;

/// 4-byte buffer. An IPv4 internet protocol address.
pub type IPv4Address = crate::IPv4Address;

/// 16-byte buffer. An IPv6 internet protocol address.
pub type IPv6Address = crate::IPv6Address;

pub type PhysicalAddress = UInt64;

pub type VirtualAddress = UInt64;