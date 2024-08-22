#[derive(Clone, PartialEq, Eq, Hash)] 
pub enum ActionState {
    Pending,
    Processing,
    Done,
    Failed(String),
}