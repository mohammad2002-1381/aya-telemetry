use aya::{Ebpf, maps::RingBuf};
use aya_telemetry_common::TelemetryEvent;
use tokio::io::unix::AsyncFd;

use crate::{evaluator::evaluate_event, event::{action::ActionType, context::EventContext, utilities::parse_string}, system::utilities::{get_ppid, get_process_path}};

pub trait EbpfLoopRun {
    fn run_loop(&mut self) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

impl EbpfLoopRun for Ebpf {
    async fn run_loop(&mut self) -> anyhow::Result<()> {
        // set up the Ring Buffer reader (map EVENTS of bpf into ring_buf)
        let ring_buf = RingBuf::try_from(self.map_mut("EVENTS").unwrap())?;

        // listening to ring buffer async
        let mut poll = AsyncFd::new(ring_buf)?;

        // async loop for read events
        loop {
            // waiting for data in buffer
            let mut guard = poll.readable_mut().await?;

            // getting access to actual buffer data
            let ring_buf = guard.get_inner_mut();

            while let Some(item) = ring_buf.next() {
                // convert raw bytes from kernel to our TelemetryEvent struct
                let event: TelemetryEvent = unsafe {
                    std::ptr::read(item.as_ptr() as *const _)
                };

                // gather all data
                let comm = parse_string(&event.comm);
                let filename = parse_string(&event.filename);
                let action_type = ActionType::from(event.action);
                let ppid = get_ppid(event.pid);
                let process_path = get_process_path(event.pid);

                // build the Context
                let ctx = EventContext {
                    pid: event.pid,
                    ppid,
                    action_type,
                    comm,
                    process_path,
                    filename: filename.clone(),
                };

                // evaluate policy to get the correct Strategy Handler
                let action_handler = evaluate_event(&ctx.action_type, &ctx.filename);

                // invoke the action (Ignore, LogOnly, or KillProcess)
                if let Err(e) = action_handler.into_handler().invoke(&ctx) {
                    eprintln!("Action handler failed for PID {}: {}", ctx.pid, e);
                }
            }
            guard.clear_ready();
        }
    }
}