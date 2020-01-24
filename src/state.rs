use data_structure::{Table, TableSize};
use minimax_strategy::Actor;
use minimax_strategy::State;
use std::collections::HashMap;

const FIELD_SIZE: TableSize = TableSize::new(6, 6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Geister {
    Blue,
    Red,
}

impl Geister {
    pub fn initial_count(&self) -> usize {
        match self {
            Geister::Blue => 4,
            Geister::Red => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnedGeister {
    geister: Geister,
    owner: Actor,
}

impl OwnedGeister {
    pub fn new(geister: Geister, owner: Actor) -> Self {
        Self { geister, owner }
    }
}

#[derive(Clone)]
pub struct GeisterState {
    pub lattices: Table<Option<OwnedGeister>>,
    pub killed_geister_counts: HashMap<OwnedGeister, usize>,
    pub actor_of_cleared_geister: Option<Actor>,
}

impl GeisterState {
    pub fn new_empty() -> Self {
        let mut killed_geister_counts = HashMap::with_capacity(2 * 2);
        for &geister in [Geister::Red, Geister::Blue].iter() {
            for &actor in [Actor::Agent, Actor::Other].iter() {
                let owned_geister = OwnedGeister::new(geister, actor);
                killed_geister_counts.insert(owned_geister, 0);
            }
        }
        Self {
            lattices: Table::from_fill(None, FIELD_SIZE),
            killed_geister_counts,
            actor_of_cleared_geister: None,
        }
    }

    pub fn winner(&self) -> Option<Actor> {
        if let Some(winner) = self.actor_of_cleared_geister {
            return Some(winner);
        }
        for actor in vec![Actor::Agent, Actor::Other].into_iter() {
            if self
                .killed_geister_counts
                .get(&OwnedGeister::new(Geister::Red, actor))
                == Some(&Geister::Red.initial_count())
            {
                return Some(actor);
            } else if self
                .killed_geister_counts
                .get(&OwnedGeister::new(Geister::Blue, actor))
                == Some(&Geister::Blue.initial_count())
            {
                return Some(actor.opponent());
            }
        }
        None
    }
}

impl State for GeisterState {
    fn is_game_over(&self) -> bool {
        self.winner().is_some()
    }
}
