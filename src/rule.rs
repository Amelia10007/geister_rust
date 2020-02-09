use crate::{
    Geister, GeisterAction, GeisterMovement, GeisterState, OwnedGeister, AVAILABLE_ACTIONS,
    FIELD_SIZE, INITIAL_GEISTER_COUNT,
};
use data_structure::{Pair, TableIndex};
use minimax_strategy::{actors, Action, Actor, Rule};

enum GeisterStateAfterAction {
    OnField(TableIndex),
    Clear,
}

pub struct GeisterRule {}

impl GeisterRule {
    /// 指定した状態におけるゲームの勝者`actor`を`Some(actor)`として返す．
    /// 勝者が決定してない場合は`None`を返す．
    pub fn winner_of(state: &GeisterState) -> Option<Actor> {
        if let Some(winner) = state.actor_of_cleared_geister {
            return Some(winner);
        }
        for &actor in actors().iter() {
            if state.killed_geister_count(OwnedGeister::new(Geister::Evil, actor))
                == INITIAL_GEISTER_COUNT
            {
                return Some(actor);
            } else if state.killed_geister_count(OwnedGeister::new(Geister::Holy, actor))
                == INITIAL_GEISTER_COUNT
            {
                return Some(actor.opponent());
            }
        }
        None
    }
}

impl Rule for GeisterRule {
    type S = GeisterState;

    type A = GeisterAction;

    type ActionIterator = Vec<GeisterAction>;

    fn is_game_over(state: &GeisterState) -> bool {
        Self::winner_of(state).is_some()
    }

    fn iterate_available_actions(state: &GeisterState, actor: Actor) -> Self::ActionIterator {
        state
            // フィールド上の，自分の👻がいる位置を求める
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
            // 👻の全動作のうち，実行可能なものだけを求める
            .flat_map(|owned_geister_position| {
                AVAILABLE_ACTIONS
                    .iter()
                    .map(move |&movement_candidate| {
                        GeisterAction::new(owned_geister_position, movement_candidate, actor)
                    })
                    .filter(|action_candidate| {
                        get_geister_state_after_action(state, &action_candidate).is_some()
                    })
            })
            // ここでヒープに実行可能な行動を保存しているが，
            // 本当はパフォーマンスの関係上iteratorのまま返したい．
            .collect::<Vec<_>>()
    }

    fn translate_state(state: &GeisterState, action: &GeisterAction) -> GeisterState {
        let mut new_state = state.clone();

        match get_geister_state_after_action(state, action)
            .expect("translate_state() accepts only available action.")
        {
            GeisterStateAfterAction::OnField(position_after_movement) => {
                // 自分の👻を他の自分の👻と同じ位置に移動させることは，ルール上あり得ない
                debug_assert_ne!(
                    Some(action.actor()),
                    new_state.lattices[position_after_movement].map(|l| l.owner)
                );
                new_state.kill_geister_at(position_after_movement);

                // 👻の移動
                let moved_geister = new_state.lattices[action.target_geister_position];
                new_state.lattices[position_after_movement] = moved_geister;
            }
            GeisterStateAfterAction::Clear => {
                new_state.actor_of_cleared_geister = Some(action.actor());
            }
        }
        // 元々👻がいたマスには何もいなくなる
        new_state.lattices[action.target_geister_position] = None;

        new_state
    }
}

/// 指定したエージェントの👻が上がれる位置を返す．
pub fn clearable_position_of(actor: Actor) -> TableIndex {
    match actor {
        Actor::First => TableIndex::new(0, 0),
        Actor::Second => FIELD_SIZE - TableIndex::new(1, 1),
    }
}

/// 指定した位置にいる👻が指定した移動をした後の位置`position`を`Some(position)`として返す．
/// ただし，👻が上がった場合は`Some(clear)`を返す．
/// また，指定した行動がルール上取れない場合は`None`を返す．
fn get_geister_state_after_action(
    state: &GeisterState,
    action: &GeisterAction,
) -> Option<GeisterStateAfterAction> {
    match action.geister_movement {
        GeisterMovement::Direction(d) => {
            // 移動後の👻の位置を計算
            let position_after_movement = {
                let p = action.target_geister_position.try_cast::<isize>().ok()?;
                let position_after_movement = p + d;
                position_after_movement.try_cast().ok()?
            };
            // 移動後の位置がフィールドからはみ出ていたり，移動後の位置に自分の他の👻がいる場合は移動できない
            if state.lattices.is_valid_index(position_after_movement)
                && state.lattices[position_after_movement].map(|owned_geister| owned_geister.owner)
                    != Some(action.actor())
            {
                Some(GeisterStateAfterAction::OnField(position_after_movement))
            } else {
                None
            }
        }
        GeisterMovement::Clear => {
            // 移動対象の👻が善良な👻で，上がれる位置に存在するなら上がれる
            let movement_geister = state.lattices[action.target_geister_position]
                .expect("Geister must exit")
                .geister;
            if movement_geister == Geister::Holy
                && action.target_geister_position == clearable_position_of(action.actor())
            {
                Some(GeisterStateAfterAction::Clear)
            } else {
                None
            }
        }
    }
}
