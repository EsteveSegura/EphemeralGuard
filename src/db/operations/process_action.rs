use crate::db::operations::action_queue::ActionQueue;
use crate::db::operations::action_type::ActionType;
use crate::db::operations::action_state::ActionState;
use crate::db::storage::principal_store::PrincipalStore;
use crate::db::models::secret_data::{SecretData};
use crate::db::operations::{create, read, delete};

pub struct ActionResult {
    pub state: ActionState,
    pub data: Option<SecretData>,
    pub message: Option<String>,
}

pub fn process_next_action(queue: &mut ActionQueue, store: &mut PrincipalStore) -> ActionResult {
    if let Some(mut action) = queue.dequeue() {
        action.state = ActionState::Processing;

        let result = match action.action_type {
            ActionType::Create { payload, expiration_date } => {
                let secret = create::create_secret(store, &payload, expiration_date);
                action.state = ActionState::Done;
                ActionResult {
                    state: ActionState::Done,
                    data: Some(secret.clone()),
                    message: Some(format!("Created secret with ID: {}", secret.id)),
                }
            },
            ActionType::Read { id } => {
                match read::read_secret(store, &id) {
                    Some(secret) => {
                        action.state = ActionState::Done;
                        ActionResult {
                            state: ActionState::Done,
                            data: Some(secret.clone()),
                            message: Some(format!("Read secret with payload: {}", secret.decrypt())),
                        }
                    },
                    None => {
                        action.state = ActionState::Failed("Secret not found or expired".to_string());
                        ActionResult {
                            state: action.state.clone(),
                            data: None,
                            message: Some("Failed to read secret: Secret not found or expired".to_string()),
                        }
                    }
                }
            },
            ActionType::Delete { id } => {
                if delete::delete_secret(store, &id) {
                    action.state = ActionState::Done;
                    ActionResult {
                        state: ActionState::Done,
                        data: None,
                        message: Some(format!("Deleted secret with ID: {}", id)),
                    }
                } else {
                    action.state = ActionState::Failed("Failed to delete secret".to_string());
                    ActionResult {
                        state: action.state.clone(),
                        data: None,
                        message: Some("Failed to delete secret".to_string()),
                    }
                }
            },
        };

        result
    } else {
        ActionResult {
            state: ActionState::Failed("No action to process".to_string()),
            data: None,
            message: Some("No action to process".to_string()),
        }
    }
}
