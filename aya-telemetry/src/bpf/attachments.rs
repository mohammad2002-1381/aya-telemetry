use anyhow::Context;
use aya::{Ebpf, programs::KProbe};

pub trait AttachOpen {
    fn attach_open(&mut self) -> anyhow::Result<()>;
}

pub trait AttachWrite {
    fn attach_write(&mut self) -> anyhow::Result<()>;
}

pub trait AttachUnlink {
    fn attach_unlink(&mut self) -> anyhow::Result<()>;
}

impl AttachOpen for Ebpf {
    fn attach_open(&mut self) -> anyhow::Result<()> {
        let program_open: &mut KProbe = self.program_mut("trace_open").unwrap().try_into()?;
        program_open.load()?;
        program_open.attach("do_sys_openat2", 0).context("failed to attach open kprobe")?;
        Ok(())
    }
}

impl AttachWrite for Ebpf {
    fn attach_write(&mut self) -> anyhow::Result<()> {
        let program_write: &mut KProbe = self.program_mut("trace_write").unwrap().try_into()?;
        program_write.load()?;
        program_write.attach("vfs_write", 0).context("failed to attach write kprobe")?;
        Ok(())
    }
}

impl AttachUnlink for Ebpf {
    fn attach_unlink(&mut self) -> anyhow::Result<()> {
        let program_unlink: &mut KProbe = self.program_mut("trace_unlink").unwrap().try_into()?;
        program_unlink.load()?;
        program_unlink.attach("do_unlinkat", 0).context("failed to attach unlink kprobe")?;
        Ok(())
    }
}