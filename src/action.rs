use data_structure::Pair;
use data_structure::TableIndex;
use minimax_strategy::{Action, Actor};

/// フィールド上の👻の動作を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeisterMovement {
    /// 指定した量だけフィールドを動く．
    Direction(Pair<isize>),
    /// フィールドから上がる．
    Clear,
}

/// ゲームGeisterにおけるエージェントの行動を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeisterAction {
    /// 動かす対象となる👻が元々存在する位置．
    pub target_geister_position: TableIndex,
    pub geister_movement: GeisterMovement,
    pub actor: Actor,
}

impl GeisterAction {
    pub fn new(
        target_geister_position: TableIndex,
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

pub const AVAILABLE_ACTIONS: [GeisterMovement; 5] = [
    GeisterMovement::Direction(Pair::new(1, 0)),
    GeisterMovement::Direction(Pair::new(-1, 0)),
    GeisterMovement::Direction(Pair::new(0, 1)),
    GeisterMovement::Direction(Pair::new(0, -1)),
    GeisterMovement::Clear,
];
