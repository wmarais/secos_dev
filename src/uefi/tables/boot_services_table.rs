use crate::uefi::{SystemTable, types::*};
use crate::uefi::tables::TableHeader;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BootServicesTable {
  pub header: TableHeader,

  // Task Priorit Services
  pub raise_tpl: RaiseTPL,
  pub restore_tpl: RestoreTPL,

  // Memory Services
  pub allocate_pages: AllocatePages,
  pub free_pages: FreePages,
  pub get_memory_map: GetMemoryMap,
  pub allocate_pool: AllocatePool,
  pub free_pool: FreePool,

  // Event & Timer Services
  pub create_event: CreateEvent,
  pub set_timer: SetTimer,
  pub wait_for_event: WaitForEvent,
  pub signal_event: SignalEvent,
  pub close_event: CloseEvent,
  pub check_event: CheckEvent,

  // Protocol Handler Services
  pub install_protocol_interface: InstallProtocolInterface,
  pub reinstall_protocol_interface: ReinstallProtocolInterface,
  pub uninstall_protocol_interface: UninstallProtocolInterface,
  pub handle_protocol: HandleProtocol,
  _reserved: *mut Void,
  pub register_protocol_notify: RegisterProtocolNotify,
  pub locate_handle: LocateHandle,
  pub locate_device_path: LocateDevicePath,
  pub install_configuration_table: InstallConfigurationTable,

  // Image Services
  pub load_image: LoadImage,
  pub start_image: StartImage,
  pub exit: Exit,
  pub unload_image: UnloadImage,
  pub exit_boot_services: ExitBootServices,

  // Miscellaneous Services
  pub get_next_monotonic_count: GetNextMonotonicCount,
  pub stall: Stall,
  pub set_watchdog_timer: SetWatchdogTimer,

  // Driver Support Services
  pub connect_controller: ConnectController,
  pub disconnect_controller: DisconnectController,

  // Open and close protocol services
  pub open_protocol: OpenProtocol,
  pub close_protocol: CloseProtocol,
  pub open_protocol_information: OpenProtocolInformation,

  // Library Services
  pub protocols_per_handle: ProtocolsPerHandle,
  pub locate_handle_buffer: LocateHandleBuffer,
  pub locate_protocol: LocateProtocol,
  pub install_multiple_protocol_interfaces: InstallMultipleProtocolInterfaces,
  pub uninstall_multiple_protocol_interfaces: UninstallMultipleProtocolInterfaces,

  // 32-bit CRC Services
  pub calculate_crc32: CalculateCRC32,

  // Miscellaneous Services
  pub copy_mem: CopyMem,
  pub set_mem: SetMem,
  pub create_event_ex: CreateEventEx

}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DevicePathProtocol {
  pub device_type: UInt8,
  pub sub_type: u8,
  pub length: [u8; 2]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpenProtocolInformationEntry {
  pub angent_handle: Handle,
  pub controller_handle: Handle,
  pub attributes: u32,
  pub open_count: u32
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TaskPriorityLevel {
  Application = 4,
  Callback = 8,
  Notify = 16,
  HighLevel = 31
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum AllocateType{
  AllocateAnyPages,
  AllocateMaxAddress,
  AllocateAddress,
  MaxAllocateType
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MemoryType {
  EfiReservedMemoryType,
  EfiLoaderCode,
  EfiLoaderData,
  EfiBootServicesCode,
  EfiBootServicesData,
  EfiRuntimeServicesCode,
  EfiRuntimeServicesData,
  EfiConventionalMemory,
  EfiUnusableMemory,
  EfiACPIReclaimMemory,
  EfiACPIMemoryNVS,
  EfiMemoryMappedIO,
  EfiMemoryMappedIOPortSpace,
  EfiPalCode,
  EfiPersistentMemory,
  EfiUnacceptedMemoryType,
  EfiMaxMemoryType
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TimerDelay {
  Cancel,
  Periodic,
  Relative
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum InterfaceType {
  NativeInterface
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum LocateSearchType {
  AllHandles,
  ByRegisterNotify,
  ByProtocol
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryDescriptor {
  memory_type: u32,
  physical_start: PhysicalAddress,
  virtual_start: VirtualAddress,
  number_of_pages: u64,
  attribute: u64
}

type CreateEvent = extern "C" fn(
  event_type: u32,
  notify_tpl: TaskPriorityLevel,
  notify_function: EventNotify,
  notif_context: *mut Void,
  event: *mut Event
) -> Status;

type EventNotify = extern "C" fn(
  event: Event, context: *mut Void
);

type CreateEventEx = extern "C" fn(
  event_type: u32,
  notify_tpl: TaskPriorityLevel,
  notify_function: EventNotify,
  notify_context: *mut Void,
  event_group: *mut GUID,
  event: *mut Event
) -> Status;

type CloseEvent = extern "C" fn(
  event: Event
) -> Status;

type SignalEvent = extern "C" fn(
  event: Event
) -> Status;

type WaitForEvent = extern "C" fn(
  number_of_events: usize,
  event: *mut Event,
  index: *mut usize
) -> Status;

type CheckEvent = extern "C" fn(
  event: Event
) -> Status;

type SetTimer = extern "C" fn(
  event: Event,
  timer_type: TimerDelay,
  trigger_time: u64
) -> Status;

type RaiseTPL = extern "C" fn(
  new_tpl: TaskPriorityLevel
) -> TaskPriorityLevel;

type RestoreTPL = extern "C" fn(
  old_tpl: TaskPriorityLevel
);

type AllocatePages = extern "C" fn(
  allocate_type: AllocateType,
  memory_type: MemoryType,
  pages: usize,
  memory: PhysicalAddress
) -> Status;

type FreePages = extern "C" fn(
  memory: PhysicalAddress,
  pages: usize
) -> Status;

type GetMemoryMap = extern "C" fn(
  memory_map_size: *mut usize,
  memory_map: *mut MemoryDescriptor,
  map_key: *mut usize,
  descriptor_size: *mut usize,
  descriptor_version: u32
) -> Status;

type AllocatePool = extern "C" fn(
  pool_type: MemoryType,
  size: usize,
  buffer: *mut *mut Void
) -> Status;

type FreePool = extern "C" fn(
  buffer: *mut Void
) -> Status;

type InstallProtocolInterface = extern "C" fn(
  handle: *mut Handle, 
  protocol: *mut GUID,
  interface_type: InterfaceType,
  interface: *mut Void
) -> Status;

type UninstallProtocolInterface = extern "C" fn(
  handle: Handle,
  protocol: *mut GUID,
  interface: *mut Void
) -> Status;

type ReinstallProtocolInterface = extern "C" fn(
  handle: Handle,
  protocol: *mut GUID,
  old_interface: *mut Void,
  new_interface: *mut Void
) -> Status;

type RegisterProtocolNotify = extern "C" fn(
  protocol: *mut GUID,
  event: Event,
  registration: *mut *mut Void
) -> Status;

type LocateHandle = extern "C" fn(
  search_type: LocateSearchType,
  protocol: *mut GUID,
  search_key: *mut Void,
  buffer_size: *mut usize,
  buffer: *mut Handle
) -> Status;

type HandleProtocol = extern "C" fn(
  handle: Handle,
  protocol: *mut GUID,
  interface: *mut *mut Void
) -> Status;

type LocateDevicePath = extern "C" fn(
  protocol: *mut GUID,
  device_path: *mut *mut DevicePathProtocol,
  device: *mut Handle
) -> Status;

type OpenProtocol = extern "C" fn(
  handle: Handle,
  protocol: *mut GUID,
  interface: *mut *mut Void,
  agent_handle: Handle,
  controller_handle: Handle,
  attributes: u32
) -> Status;

type CloseProtocol = extern "C" fn(
  handle: Handle,
  protocol: *mut GUID,
  agent_handle: Handle,
  controller_handle: Handle
) -> Status;

type OpenProtocolInformation = extern "C" fn(
  handle: Handle,
  protocol: *mut GUID,
  entry_buffer: *mut *mut OpenProtocolInformationEntry,
  entry_count: *mut usize
) -> Status;

type ConnectController = extern "C" fn(
  controller_handle: Handle,
  driver_image_handler: *mut Handle,
  remaining_device_path: *mut DevicePathProtocol,
  recursive: bool
) -> Status;

type DisconnectController =  extern "C" fn(
  controller_handle: Handle,
  driver_image_handle: Handle,
  child_handle: Handle
) -> Status;

type ProtocolsPerHandle = extern "C" fn(
  handle: Handle,
  protocol_buffer: *mut *mut GUID,
  protocol_buffer_count: *mut usize
) -> Status;

type LocateHandleBuffer = extern "C" fn(
  search_type: LocateSearchType,
  protocol: *mut GUID,
  search_key: *mut Void,
  no_handles: *mut usize,
  buffer: *mut *mut Handle
) -> Status;

type LocateProtocol = extern "C" fn(
  protocol: *mut GUID,
  registration: *mut Void,
  interface: *mut *mut Void
) -> Status;

type InstallMultipleProtocolInterfaces = extern "C" fn(
  handle: *mut Handle
) -> Status;

type UninstallMultipleProtocolInterfaces = extern "C" fn(
  handle: Handle
) -> Status;

type LoadImage = extern "C" fn(
  boot_polic: bool,
  parent_image_handle: Handle,
  device_path: *mut DevicePathProtocol,
  source_buffer: *mut Void,
  source_size: usize,
  image_handle: *mut Handle
) -> Status;

type StartImage = extern "C" fn(
  image_handle: Handle,
  exit_data_size: *mut usize,
  exit_data: *mut *mut u16
) -> Status;

type UnloadImage = extern "C" fn(
  image_handle: Handle
) -> Status;

pub type ImageEntryPoint = extern "C" fn(
  image_handle: Handle,
  system_table: *mut SystemTable
) -> Status;

type Exit = extern "C" fn(
  image_handle: Handle,
  exit_status: Status,
  exit_data_size: usize,
  exit_data: *mut u16
) -> Status;

type ExitBootServices = extern "C" fn(
  image_handle: Handle,
  map_key: usize
) -> Status;

type SetWatchdogTimer = extern "C" fn(
  timeout: usize,
  wathdog_code: u64,
  data_size: usize,
  watchdog_data: *mut u16
) -> Status;

type Stall = extern "C" fn(
  microseconds: usize
) -> Status;

type CopyMem = extern "C" fn(
  destination: *mut Void,
  source: *mut Void,
  length: usize
) -> Status;

type SetMem = extern "C" fn(
  buffer: *mut Void,
  size: usize,
  value: u8);

type GetNextMonotonicCount = extern "C" fn(
  count: *mut u64
) -> Status;

type InstallConfigurationTable = extern "C" fn(
  guid: *mut GUID,
  table: *mut Void
) -> Status;

type CalculateCRC32 = extern "C" fn(data: *mut Void,
  data_size: usize,
  crc32: *mut u32
) -> Status;
