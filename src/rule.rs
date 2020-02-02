use crate::{GeisterAction, GeisterMovement, GeisterState};
use data_structure::{Pair, TableIndex};
use minimax_strategy::{Action, Actor, Rule};

enum GeisterStateAfterAction {
    OnField(TableIndex),
    Clear,
}

pub struct GeisterRule {}

impl Rule<GeisterState, GeisterAction> for GeisterRule {
    type ActionIterator = Vec<GeisterAction>;

    fn iterate_available_actions(
        &self,
        state: &GeisterState,
        actor: Actor,
    ) -> Self::ActionIterator {
        const MOVEMENT_CANDIDATES: [GeisterMovement; 5] = [
            GeisterMovement::Direction(Pair::new(1, 0)),
            GeisterMovement::Direction(Pair::new(-1, 0)),
            GeisterMovement::Direction(Pair::new(0, 1)),
            GeisterMovement::Direction(Pair::new(0, -1)),
            GeisterMovement::Clear,
        ];

        state
            .lattices
            .iter_row()
            .enumerate()
            .flat_map(|(y, iter_row)| {
                iter_row
                    .iter()
                    .enumerate()
                    .filter(|(_x, owned_geister)| owned_geister.map(|g| g.owner) == Some(actor))
                    .map(move |(x, _owned_geister)| Pair::new(x, y))
            })
            .flat_map(|owned_geister_position| {
                MOVEMENT_CANDIDATES
                    .iter()
                    .map(move |&movement_candidate| {
                        GeisterAction::new(owned_geister_position, movement_candidate, actor)
                    })
                    .filter(|action_candidate| {
                        get_geister_state_after_action(state, &action_candidate).is_some()
                    })
            })
            .collect()
    }

    fn translate_state(&self, state: &GeisterState, action: &GeisterAction) -> GeisterState {
        let mut new_state = state.clone();

        match get_geister_state_after_action(state, action).unwrap() {
            GeisterStateAfterAction::OnField(position_after_movement) => {
                // 相手プレイヤーの👻と接触した場合は，その👻をフィールドから取り除く
                if let Some(g) = new_state.lattices[position_after_movement] {
                    // 自分の👻と接触することはルール上あり得ない
                    debug_assert_eq!(action.actor().opponent(), g.owner);
                    // フィールドから取り除かれた👻の数をカウントアップ．この数は勝敗判定に利用する．
                    if let Some(count) = new_state.killed_geister_counts.get_mut(&g) {
                        *count += 1;
                    } else {
                        new_state.killed_geister_counts.insert(g, 1).unwrap();
                    }
                }

                // 👻の移動
                let moved_geister = new_state.lattices[action.target_geister_position];
                new_state.lattices[position_after_movement] = moved_geister;
            }
            GeisterStateAfterAction::Clear => {
                new_state.actor_of_cleared_geister = Some(action.actor())
            }
        }

        // 元々👻がいたマスには何もいなくなる
        new_state.lattices[action.target_geister_position] = None;

        new_state
    }
}

pub fn clearable_position_of(state: &GeisterState, actor: Actor) -> TableIndex {
    match actor {
        Actor::Agent => TableIndex::new(state.lattices.width() - 1, state.lattices.height() - 1),
        Actor::Other => TableIndex::new(0, 0),
    }
}

/// 指定した位置にいる👻が指定した移動をした後の位置`position`を`Some(position)`として返す．
/// ただし，👻が上がった場合は`None`を返す．
fn get_geister_state_after_action(
    state: &GeisterState,
    action: &GeisterAction,
) -> Option<GeisterStateAfterAction> {
    let lattices = &state.lattices;

    match action.geister_movement {
        GeisterMovement::Direction(d) => {
            let p = action.target_geister_position.try_cast::<isize>().ok()?;
            let addition = p + d;
            let addition = addition.try_cast().ok()?;
            // 移動後の位置がフィールドからはみ出ていたり，移動後の位置に自分の他の👻がいる場合は移動できない
            if lattices.is_valid_index(addition)
                && lattices[addition].map(|owned_geister| owned_geister.owner)
                    != Some(action.actor())
            {
                Some(GeisterStateAfterAction::OnField(addition))
            } else {
                None
            }
        }
        GeisterMovement::Clear => {
            if action.target_geister_position == clearable_position_of(state, action.actor()) {
                Some(GeisterStateAfterAction::Clear)
            } else {
                None
            }
        }
    }
}
