use std::fmt;

pub enum ActionType {
    Create {
        payload: String,
        expiration_date: u64,
    },
    Read {
        id: String
    },
    Delete {
        id: String
    }
}

impl fmt::Debug for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Create { payload, expiration_date } => {
                f.debug_struct("Create")
                    .field("payload", payload)
                    .field("expiration_date", expiration_date)
                    .finish()
            }
            ActionType::Read { id } => {
                f.debug_struct("Read")
                    .field("id", id)
                    .finish()
            }
            ActionType::Delete { id } => {
                f.debug_struct("Delete")
                    .field("id", id)
                    .finish()
            }
        }
    }
}