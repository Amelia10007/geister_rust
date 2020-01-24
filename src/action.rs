use data_structure::Pair;
use minimax_strategy::{Action, Actor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeisterMovement {
    Left,
    Right,
    Up,
    Down,
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeisterAction {
    pub target_geister_position: Pair<usize>,
    pub geister_movement: GeisterMovement,
    pub actor: Actor,
}

impl GeisterAction {
    pub fn new(
        target_geister_position: Pair<usize>,
        geister_movement: GeisterMovement,
        actor: Actor,
    ) -> Self {
        Self {
            target_geister_position,
            geister_movement,
            actor,
        }
    }
}

impl Action for GeisterAction {
    fn actor(&self) -> Actor {
        self.actor
    }
}
