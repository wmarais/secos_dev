// use core::ffi::c_void;
// use crate::GUID;
// use crate::uefi::{Event, Status};

use crate::uefi::{
  UIntN, types::{
    Boolean, Char16, Event, Int32, Status, UInt16, UInt32, Void
  }
};

#[repr(C)]
#[derive(Debug)]
pub struct SimpleTextInputEx {
  pub reset: InputResetEx,
  pub read_key_stroke_ex: InputReadKeyStrokeEx,
  pub wait_for_key_ex: Event,
  pub set_state: SetStateEx,
  pub register_key_notify: RegisterKeyNotifyEx,
  pub unregister_key_notify: UnregisterKeyNotifyEx
}

/// Clear the contents of any input queues resident in memory used for buffering
/// keystroke data and put the input stream in a known empty state.
type InputResetEx = extern "C" fn(
  this: *mut SimpleTextInputEx, 
  extended_verification: Boolean
) -> Status;

/// Reads the next keystroke from the input device. If there is no pending 
/// keystroke the function returns EFI_NOT_READY . If there is a pending 
/// keystroke, then KeyData.Key.ScanCode is the EFI scan code.
type InputReadKeyStrokeEx = extern "C" fn(
  this: *mut SimpleTextInputEx,
  key_data: *mut KeyData
) -> Status;

/// Allows the input device hardware to have state settings adjusted. By calling
/// the SetState() function with the EFI_KEY_STATE_EXPOSED bit active in the 
/// KeyToggleState parameter, this will enable the ReadKeyStrokeEx function to 
/// return incomplete keystrokes such as the holding down of certain keys which 
/// are expressed as a part of KeyState when there is no Key data.
type SetStateEx = extern "C" fn(
  this: *mut SimpleTextInputEx,
  key_toggle_state: *mut KeyToggleState
) -> Status;

/// Registers a function which will be called when a specified keystroke will 
/// occur. The keystroke being specified can be for any combination of 
/// KeyData.Key or KeyData.KeyState information.
type RegisterKeyNotifyEx = extern "C" fn(
  this: *mut SimpleTextInputEx,
  key_data: *mut KeyData,
  key_notification_function: KeyNotifyFunction,
  notify_handle: *mut *mut Void
) -> Status;

/// The function to be called when the key sequence is typed specified by 
/// KeyData.
type KeyNotifyFunction = extern "C" fn(
  key_data: *mut KeyData
) -> Status;

/// Removes the notification which was previously registered.                                        
type UnregisterKeyNotifyEx = extern "C" fn(
  this: *mut SimpleTextInputEx,
  notification_handle: *mut Void
) -> Status;


#[repr(C)]
#[derive(Debug)]
pub struct SimpleTextInput {
  pub reset: InputReset,
  pub read_key_stroke: ReadKeyStroke,
  pub wait_for_key: Event
}


/// Clear the contents of any input queues resident in memory used for buffering
/// keystroke data and put the input stream in a known empty state.
type InputReset = extern "C" fn(
  this: *mut SimpleTextInput,
  extended_verification: bool
) -> Status;

/// The ReadKeyStroke() function reads the next keystroke from the input device.
/// If there is no pending keystroke the function returns EFI_NOT_READY. 
type ReadKeyStroke = extern "C" fn(
  this: *mut SimpleTextInput, 
  key: *mut InputKey
) -> Status;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputKey {
  pub scan_code: UInt16,
  pub unicode_char: Char16
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct KeyToggleState(u8);
impl KeyToggleState {
  pub const TOGGLE_STATE_VALID: Self = Self { 0: 0x80 };
  pub const KEY_STATE_EXPOSED: Self = Self { 0: 0x40 };
  pub const SCROLL_LOCK_ACTIVE: Self = Self { 0: 0x01 };
  pub const NUM_LOCK_ACTIVE: Self = Self { 0: 0x02 };
  pub const CAPS_LOCK_ACTIVE: Self = Self { 0: 0x04 };
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct KeyShiftState(UInt32);

impl KeyShiftState {
  pub const SHIFT_STATE_VALID: Self = Self { 0: 0x80000000 };
  pub const RIGHT_SHIFT_PRESSED: Self = Self { 0: 0x00000001 };
  pub const LEFT_SHIFT_PRESSED: Self = Self { 0: 0x00000002 };
  pub const RIGHT_CONTROL_PRESSED: Self = Self { 0: 0x00000004 };
  pub const LEFT_CONTROL_PRESSED: Self = Self { 0: 0x00000008 };
  pub const RIGHT_ALT_PRESSED: Self = Self { 0: 0x00000010 };
  pub const LEFT_ALT_PRESSED: Self = Self { 0: 0x00000020 };
  pub const RIGHT_LOGO_PRESSED: Self = Self { 0: 0x00000040 };
  pub const LEFT_LOGO_PRESSED: Self = Self { 0: 0x00000080 };
  pub const MENU_KEY_PRESSED: Self = Self { 0: 0x00000100 };
  pub const SYS_REQ_PRESSED: Self = Self { 0: 0x00000200 };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyState {
  pub shift_state: KeyShiftState,
  pub toggle_state: KeyToggleState
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyData {
  pub key: InputKey
}


//==================================================================================================
// 12.3.4. ConsoleOut or StandardError
//==================================================================================================

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SimpleTextOutput {
  reset: TextReset,
  output_string: TextOutputString,
  test_string: TextTestString,
  query_mode: TextQueryMode,
  set_mode: TextSetMode,
  set_attribute: TextSetAttribute,
  clear_screen: TextClearScreen,
  set_cursor_position: TextSetCursorPosition,
  enable_cursor: TextEnableCursor,
  mode: *mut SimpleTextOutputMode
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SimpleTextOutputMode {
  pub max_mode: Int32,
  pub mode: Int32,
  pub attribute: Int32,
  pub cursor_column: Int32,
  pub cursor_row: Int32,
  pub cursor_visible: Boolean
}

/// Resets the text output device hardware. The cursor position is set to 
/// (0, 0), and the screen is cleared to the default background color for the 
/// output device.
type TextReset = extern "C" fn(
  this: *mut SimpleTextOutput, 
  extended_verification: Boolean
) -> Status;

/// Writes a string to the output device.
type TextOutputString = extern "C" fn(
  this: *mut SimpleTextOutput,
  string: *mut Char16
) -> Status;

/// Verifies that all characters in a string can be output to the target device.
type TextTestString = extern "C" fn(
  this: *mut SimpleTextOutput,
  string: *mut Char16
) -> Status;

/// Returns information for an available text mode that the output device(s) 
/// supports.
type TextQueryMode = extern "C" fn(
  this: *mut SimpleTextOutput,
  mode_number: UIntN,
  columns: *mut UIntN,
  rows: *mut UIntN
) -> Status;

/// The SetMode() function sets the output device(s) to the requested mode. On 
/// success the device is in the geometry for the requested mode, and the device
/// has been cleared to the current background color with the cursor at (0,0).
type TextSetMode = extern "C" fn(
  this: *mut SimpleTextOutput,
  mode_number: UIntN
) -> Status;

type TextSetAttribute = extern "C" fn(
  this: *mut SimpleTextOutput,
  attribute: UIntN
) -> Status;

type TextClearScreen = extern "C" fn(
  this: *mut SimpleTextOutput
) -> Status;

type TextSetCursorPosition = extern "C" fn(
  this: *mut SimpleTextOutput,
  column: UIntN,
  row: UIntN
) -> Status;

type TextEnableCursor = extern "C" fn(
  this: *mut SimpleTextOutput,
  visible: Boolean
) -> Status;

//==================================================================================================
// 12.5. Simple Pointer Protocol
//==================================================================================================




