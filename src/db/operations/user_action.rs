use super::action_type::ActionType;
use super::action_state::ActionState;

pub struct UserAction {
    pub id: u64,
    pub action_type: ActionType,
    pub state: ActionState,
}