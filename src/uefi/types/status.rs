/// This module implement the EFI Status type and codes as document at:
///   https://uefi.org/specs/UEFI/2.11/Apx_D_Status_Codes.html

use crate::uefi::UIntN; 

/// Bitmask to isolate the highest bit in a 32bit UIntN.
#[cfg(target_pointer_width = "32")]
const UIntN_HIGH_BIT: UIntN = 0x80_00_00_00;

/// Bitmask to isolate the highest bit in a 64bit UIntN.
#[cfg(target_pointer_width = "64")]
const UINTN_HIGHEST_BIT: UIntN = 0x80_00_00_00__00_00_00_00;

/// Bitmask to isolate the next highest bit in a 32bit UIntN.
#[cfg(target_pointer_width = "32")]
const UIntN_NEXT_HIGHEST_BIT: UIntN = 0x40_00_00_00;

/// Bitmask to isolate the next highest bit in a 64bit UIntN.
#[cfg(target_pointer_width = "64")]
const UINTN_NEXT_HIGHEST_BIT: UIntN = 0x40_00_00_00__00_00_00_00;

/// The two highest bit in a UIntN that determine whether the status code is an
/// EFI reserved status code or a OEM reserved status code.
const CHECK_MASK: UIntN = UINTN_HIGHEST_BIT | UINTN_NEXT_HIGHEST_BIT;

/// Generate a status code with the highest bit cleared.
macro_rules! high_bit_clear {($v:expr) => { $v }}

/// Generate a status code with the highest bit set.
macro_rules! high_bit_set {($v:expr) => { $v | UINTN_HIGHEST_BIT }}

/// EFI interfaces return an EFI_STATUS code. See EFI_STATUS Success Codes (High
/// Bit Clear), EFI_STATUS Error Codes (High Bit Set), EFI_STATUS Warning
/// Codes (High Bit Clear) list these codes for success, errors, and warnings,
/// respectively.    
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Status(UIntN);

impl Status {
  /// The range of status codes that are reserved for EFI have:
  ///   1. both the highest bits cleared, or 
  ///   2. the highest bit set and the next to highest bit cleared.
  #[inline(always)]
  pub fn is_efi_reserved_code(code: UIntN) -> bool {
    code & CHECK_MASK == 0 ||
    code & CHECK_MASK == UINTN_HIGHEST_BIT
  }

  /// The range of status codes that are reserved for OEMs have:
  ///   1. both the highest bit set, or
  ///   2. the highest cleared set and the next highest bit set.
  #[inline(always)]
  pub fn is_oem_reserved_code(code: UIntN) -> bool {
    code & CHECK_MASK == CHECK_MASK ||
    code & CHECK_MASK == UINTN_NEXT_HIGHEST_BIT
  }

  /// Success and warning codes have their highest bit clear, so all success and
  /// warning codes have positive values.
  #[inline(always)]
  pub fn is_warning(code: UIntN) -> bool {
    code & UINTN_HIGHEST_BIT == 0
  }

  /// Error codes have the highest bit set, so all error codes have negative
  /// values.
  #[inline(always)]
  pub fn is_error(code: UIntN) -> bool {
    code & UINTN_HIGHEST_BIT == UINTN_HIGHEST_BIT
  }

  /// The operation completed successfully.
  pub const SUCCESS: UIntN = high_bit_clear!(0);

  /// The image failed to load.
  pub const LOAD_ERROR: UIntN = high_bit_set!(1);

  /// A parameter was incorrect.
  pub const INVALID_PARAMETER: UIntN = high_bit_set!(2);

  /// The operation is not supported.
  pub const UNSUPPORTED: UIntN = high_bit_set!(3);

  /// The buffer was not the proper size for the request.
  pub const BAD_BUFFER_SIZE: UIntN = high_bit_set!(4);

  /// The buffer is not large enough to hold the requested data. The required 
  /// buffer size is returned in the appropriate parameter when this error 
  /// occurs.
  pub const BUFFER_TOO_SMALL: UIntN = high_bit_set!(5);

  /// There is no data pending upon return.
  pub const NOT_READY: UIntN = high_bit_set!(6);

  /// The physical device reported an error while attempting the operation.
  pub const DEVICE_ERROR: UIntN = high_bit_set!(7);

  /// The device cannot be written to.
  pub const WRITE_PROTECTED: UIntN = high_bit_set!(8);

  /// A resource has run out.
  pub const OUT_OF_RESOURCES: UIntN = high_bit_set!(9);

  /// An inconstancy was detected on the file system causing the operating to 
  /// fail.
  pub const VOLUME_CORRUPTED: UIntN = high_bit_set!(10);

  /// There is no more space on the file system.
  pub const VOLUME_FULL: UIntN = high_bit_set!(11);

  /// The device does not contain any medium to perform the operation.
  pub const NO_MEDIA: UIntN = high_bit_set!(12); 

  /// The medium in the device has changed since the last access.
  pub const MEDIA_CHANGED: UIntN = high_bit_set!(13);

  /// The item was not found.
  pub const NOT_FOUND: UIntN = high_bit_set!(14);

  /// Access was denied.
  pub const ACCESS_DENIED: UIntN = high_bit_set!(15);

  /// The server was not found or did not respond to the request.
  pub const NO_RESPONSE: UIntN = high_bit_set!(16);

  /// A mapping to a device does not exist.
  pub const NO_MAPPING: UIntN = high_bit_set!(17);

  /// The timeout time expired.
  pub const TIMEOUT: UIntN = high_bit_set!(18);

  /// The protocol has not been started.
  pub const NOT_STARTED: UIntN = high_bit_set!(19);

  /// The protocol has already been started.
  pub const ALREADY_STARTED: UIntN = high_bit_set!(20);

  /// The operation was aborted.
  pub const ABORTED: UIntN = high_bit_set!(21);

  /// An ICMP error occurred during the network operation.
  pub const ICMP_ERROR: UIntN = high_bit_set!(22);

  /// A TFTP error occurred during the network operation.
  pub const TFTP_ERROR: UIntN = high_bit_set!(23);

  /// A protocol error occurred during the network operation.
  pub const PROTOCOL_ERROR: UIntN = high_bit_set!(24);

  /// The function encountered an internal version that was incompatible with a
  /// version requested by the caller.
  pub const INCOMPATIBLE_VERSION: UIntN = high_bit_set!(25);

  /// The function was not performed due to a security violation.
  pub const SECURITY_VIOLATION: UIntN = high_bit_set!(26);

  /// A CRC error was detected.
  pub const CRC_ERROR: UIntN = high_bit_set!(27);

  /// Beginning or end of media was reached
  pub const END_OF_MEDIA: UIntN = high_bit_set!(28);

  /// The end of the file was reached.
  pub const END_OF_FILE: UIntN = high_bit_set!(31);

  /// The language specified was invalid.
  pub const INVALID_LANGUAGE: UIntN = high_bit_set!(32);

  /// The security status of the data is unknown or compromised and the data
  /// must be updated or replaced to restore a valid security status.
  pub const COMPROMISED_DATA: UIntN = high_bit_set!(33);

  /// There is an address conflict address allocation.
  pub const IP_ADDRESS_CONFLICT: UIntN = high_bit_set!(34);

  /// A HTTP error occurred during the network operation.
  pub const HTTP_ERROR: UIntN = high_bit_set!(35);

  /// The string contained one or more characters that the device could not 
  /// render and were skipped.
  pub const WARN_UNKNOWN_GLYPH: UIntN = high_bit_clear!(1);

  /// The handle was closed, but the file was not deleted.
  pub const WARN_DELETE_FAILURE: UIntN = high_bit_clear!(2);

  /// The handle was closed, but the data to the file was not flushed properly.
  pub const WARN_WRITE_FAILURE: UIntN = high_bit_clear!(3);
  
  /// The resulting buffer was too small, and the data was truncated to the 
  /// buffer size.
  pub const WARN_BUFFER_TOO_SMALL: UIntN = high_bit_clear!(4);

  /// The data has not been updated within the timeframe set by local policy for
  /// this type of data.
  pub const WARN_STALE_DATA: UIntN = high_bit_clear!(5);

  /// The resulting buffer contains UEFI-compliant file system.
  pub const WARN_FILE_SYSTEM: UIntN = high_bit_clear!(6);

  /// The operation will be processed across a system reset.
  pub const WARN_RESET_REQUIRED: UIntN = high_bit_clear!(7);
}
