pub enum Command {
    Move { x: u32, y: u32, light: bool },
    Wait { light: bool },
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Move { x, y, light } => write!(f, "{x} {y} {}", if *light { 1 } else { 0 }),
            Self::Wait { light } => write!(f, "{}", if *light { 1 } else { 0 }),
        }
    }
}
