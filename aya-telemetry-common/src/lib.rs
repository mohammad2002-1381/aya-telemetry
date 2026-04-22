#![no_std]

// for passing data between epbf and user space
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TelemetryEvent {
    pub pid: u32,
    pub ppid: u32,
    pub cgroup_id: u64,
    pub action: u8, // 0=>create/open , 1=>write/update , 2=>unlink/delete
    pub comm: [u8; 16], // process binary name
    pub filename: [u8; 256] // file name/path
}

// reading from from ring buffer
#[cfg(feature = "user")]
unsafe impl aya::Pod for TelemetryEvent {}