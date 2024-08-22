use secret_db::db::operations::action_queue::ActionQueue;
use secret_db::db::operations::action_type::ActionType;
use secret_db::db::operations::action_state::ActionState;
use secret_db::db::operations::user_action::UserAction;
use secret_db::db::operations::process_action::process_next_action;
use secret_db::db::storage::principal_store::PrincipalStore;
use secret_db::utils::time::current_timestamp;

fn main() {
    let mut store = PrincipalStore::new();
    let mut queue = ActionQueue::new();
    // Queue actions
    queue.enqueue(UserAction {
        id: 1,
        action_type: ActionType::Create { 
            payload: "my_secret_1".to_string(),
            expiration_date: current_timestamp() + 2000,
        },
        state: ActionState::Pending,
    });

    queue.enqueue(UserAction {
        id: 2,
        action_type: ActionType::Read { 
            id: "NOT_EXISTING_ID".to_string(),
        },
        state: ActionState::Pending,
    });

    queue.enqueue(UserAction {
        id: 3,
        action_type: ActionType::Read { 
            id: "4563d119".to_string(),
        },
        state: ActionState::Pending,
    });

    queue.enqueue(UserAction {
        id: 3,
        action_type: ActionType::Delete { 
            id: "4563d119".to_string(),
        },
        state: ActionState::Pending,
    });

    // Process actions
    while !queue.is_empty() {
        let result = process_next_action(&mut queue, &mut store);
        match result.state {
            ActionState::Done => {
                if let Some(secret) = result.data {
                    println!("Action completed successfully: {:?}", secret);
                } else {
                    println!("Action completed successfully");
                }
            }
            ActionState::Failed(reason) => {
                println!("Action failed: {}", reason);
            }
            _ => {}
        }
    }
}
