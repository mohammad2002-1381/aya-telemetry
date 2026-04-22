#[derive(Debug, PartialEq)]
pub enum ActionType {
    Open,
    Write,
    Delete,
    Unknown,
}

impl From<u8> for ActionType {
    fn from(val: u8) -> Self {
        match val {
            0 => ActionType::Open,
            1 => ActionType::Write,
            2 => ActionType::Delete,
            _ => ActionType::Unknown,
        }
    }
}