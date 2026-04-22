#![no_std]
#![no_main]

use aya_ebpf::{helpers::{bpf_get_current_comm, bpf_get_current_pid_tgid, bpf_probe_read_user_str_bytes, generated::bpf_get_current_cgroup_id}, macros::{kprobe, map}, maps::RingBuf, programs::ProbeContext};
use aya_telemetry_common::TelemetryEvent;

#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(256 * 1024, 0);

// ctx.arg(1) = filename pointer
#[kprobe]
pub fn trace_open(ctx: ProbeContext) -> u32 {
    // action:0 => open/create
    let filename_ptr = ctx.arg(1).unwrap_or(0 as *const u8);
    handle_event(&ctx, 0, filename_ptr)
}

#[kprobe]
pub fn trace_write(ctx: ProbeContext) -> u32 {
    // action:1 => write/update
    let filename_ptr = ctx.arg(1).unwrap_or(0 as *const u8);
    handle_event(&ctx, 1, filename_ptr)
}

#[kprobe]
pub fn trace_unlink(ctx: ProbeContext) -> u32 {
    // action:2 => unlink/delete
    let filename_ptr = ctx.arg(1).unwrap_or(0 as *const u8);
    handle_event(&ctx, 2, filename_ptr)
}

fn handle_event(_ctx: &ProbeContext, action: u8, filename_ptr: *const u8) -> u32 {
    // reserve space from ring buffer as TelemetryEvent
    if let Some(mut event_buf) = EVENTS.reserve::<TelemetryEvent>(0) {
        let mut event = TelemetryEvent {
            pid: 0,
            ppid: 0,
            cgroup_id: 0,
            action,
            comm: [0; 16],
            filename: [0; 256]
        };

        // get pid (top 32 bits are TGID/PID, bottom 32 are thread id)
        let pid_tgid = bpf_get_current_pid_tgid();
        event.pid = (pid_tgid >> 32) as u32;

        // get cgroup_id
        event.cgroup_id = unsafe { bpf_get_current_cgroup_id() };

        // get process name
        let _ = bpf_get_current_comm().unwrap_or([0; 16]);

        // get filename from user space pointer

        if !filename_ptr.is_null() {
            let _ = unsafe {
                bpf_probe_read_user_str_bytes(filename_ptr, &mut event.filename)
            };
        }

        // write to buffer and submit to user space
        event_buf.write(event);
        event_buf.submit(0);
    }
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        
    }
}