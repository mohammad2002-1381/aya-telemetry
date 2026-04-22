use anyhow::Ok;
use aya::{
    Ebpf
};
use aya_log::EbpfLogger;
use log::{info, warn};
use aya_telemetry::{
    bpf::{attachments::{AttachOpen, AttachUnlink, AttachWrite}, run::EbpfLoopRun, setup::Setup}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ex. RUST_LOG=info cargo run
    env_logger::init();

    let mut bpf = Ebpf::setup()?;

    // eBPF logger
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    bpf.attach_open()?;
    bpf.attach_write()?;
    bpf.attach_unlink()?;

    info!("waiting for Ctrl-C...");

    let _ = bpf.run_loop().await;

    Ok(())
}


#[cfg(test)]
mod tests {
    use aya_telemetry::{evaluator::{action::ActionDecision, evaluate_event}, event::action::ActionType};

    #[test]
    fn test_ignore_unrelated_files() {
        assert_eq!(evaluate_event(&ActionType::Open, "/tmp/random_file.txt"), ActionDecision::Ignore);
        assert_eq!(evaluate_event(&ActionType::Write, "/home/user/document.txt"), ActionDecision::Ignore);
    }

    #[test]
    fn test_log_only_for_monitored_dirs() {
        assert_eq!(evaluate_event(&ActionType::Write, "/var/secure/data.txt"), ActionDecision::Log);
        assert_eq!(evaluate_event(&ActionType::Open, "/home/secure_area/config.json"), ActionDecision::Log);
        assert_eq!(evaluate_event(&ActionType::Delete, "/var/secure/old_data.txt"), ActionDecision::Log);
    }

    #[test]
    fn test_kill_for_protected_writes_and_deletes() {
        assert_eq!(evaluate_event(&ActionType::Write, "/opt/protected/secret.txt"), ActionDecision::Kill);
        assert_eq!(evaluate_event(&ActionType::Delete, "/opt/protected/secret.txt"), ActionDecision::Kill);
    }

    #[test]
    fn test_allow_reads_in_protected_dir() {
        assert_eq!(evaluate_event(&ActionType::Open, "/opt/protected/secret.txt"), ActionDecision::Log);
    }
}
