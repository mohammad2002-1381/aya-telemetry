use aya::{Ebpf, include_bytes_aligned};

pub trait Setup: Sized {
    fn setup() -> Result<Self, anyhow::Error>;
}

impl Setup for Ebpf {
    fn setup() -> Result<Self, anyhow::Error> {
        #[cfg(debug_assertions)]
        let bpf = Ebpf::load(
            include_bytes_aligned!(
                "../../../target/bpfel-unknown-none/debug/aya-telemetry"
            )
        )?;

        // for release
        #[cfg(not(debug_assertions))]
        let bpf = Ebpf::load(
            include_bytes_aligned!(
                "../../../target/bpfel-unknown-none/release/aya-telemetry"
            )
        )?;

        Ok(bpf)
    }
}