use anyhow::Ok;
use crate::{event::context::EventContext, system};

#[derive(Debug, PartialEq)]
pub enum ActionDecision {
    Kill,
    Log,
    Ignore,
}

impl ActionDecision {
    pub fn into_handler(self) -> Box<dyn SecurityActionHandler> {
        match self {
            ActionDecision::Kill => Box::new(KillProcess),
            ActionDecision::Log => Box::new(LogProcess),
            ActionDecision::Ignore => Box::new(IgnoreProcess),
        }
    }
}

pub struct KillProcess;
pub struct IgnoreProcess;
pub struct LogProcess;

pub trait SecurityActionHandler: Send + Sync {
    fn invoke(&self, ctx: &EventContext) -> anyhow::Result<()>;
}

impl SecurityActionHandler for IgnoreProcess {
    fn invoke(&self, _: &EventContext) -> anyhow::Result<()> {
        Ok(())
    }
}

impl SecurityActionHandler for LogProcess {
    fn invoke(&self, ctx: &EventContext) -> anyhow::Result<()> {
        println!(
            "[{:?}] PID: {} | PPID: {} | Comm: {} | Path: {} | File: {}",
            ctx.action_type, ctx.pid, ctx.ppid, ctx.comm, ctx.process_path, ctx.filename
        );
        Ok(())
    }
}

impl SecurityActionHandler for KillProcess {
    fn invoke(&self, ctx: &EventContext) -> anyhow::Result<()> {
        println!(
            "SECURITY VIOLATION: process {} ({}) attempted to modify {}. terminating.", 
            ctx.pid, ctx.process_path, ctx.filename
        );
        
        if let Err(e) = system::utilities::kill_process(ctx.pid) {
            eprintln!("failed to terminate process {}: {}", ctx.pid, e);
        }
        
        LogProcess.invoke(ctx)?;
        
        Ok(())
    }
}