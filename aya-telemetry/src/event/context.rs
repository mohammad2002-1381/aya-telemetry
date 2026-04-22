use crate::event::action::ActionType;

pub struct EventContext {
    pub pid: u32,
    pub ppid: u32,
    pub action_type: ActionType,
    pub comm: String,
    pub process_path: String,
    pub filename: String,
}
