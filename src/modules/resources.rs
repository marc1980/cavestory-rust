#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerStatus {
    pub direction: Option<PlayerDirection>,
    pub is_jumping: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerDirection {
    Left,
    Right,
    Stopped,
}