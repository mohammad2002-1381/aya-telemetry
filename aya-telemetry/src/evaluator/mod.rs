use crate::{evaluator::action::ActionDecision, event::action::ActionType};

pub mod action;

pub fn evaluate_event(action: &ActionType, filename: &str) -> ActionDecision {
    let is_protected = filename.starts_with("/opt/protected");
    let is_monitored = filename.starts_with("/var/secure") || filename.starts_with("/home/secure_area");

    if (*action == ActionType::Write || *action == ActionType::Delete) && is_protected {
        return ActionDecision::Kill;
    }

    if is_protected || is_monitored {
        return ActionDecision::Log;
    }

    ActionDecision::Ignore
}