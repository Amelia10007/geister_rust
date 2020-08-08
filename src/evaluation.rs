use crate::*;
use itertools::Itertools;
use minimax_strategy::{
    construct_alpha_beta_strategy, Actor, AlphaBetaStrategy, Evaluator, Strategy,
};

/// ゲームGeisterのフィールドに対する利得を表す．
pub type GeisterPayoff = i32;

/// ゲームGeisterのフィールドに対する評価関数を表す．
pub struct GeisterEvaluator {}

impl Evaluator<GeisterState> for GeisterEvaluator {
    type Payoff = GeisterPayoff;

    fn evaluate_payoff_for(actor: Actor, state: &GeisterState) -> Self::Payoff {
        let payoffs = iterate_possible_states(actor, state)
            .into_iter()
            .map(|(state, count)| (evaluate_payoff_for_exact_state(actor, &state), count))
            .collect::<Vec<_>>();
        let weighted_payoff_sum = payoffs
            .iter()
            .map(|&(payoff, count)| payoff * count as GeisterPayoff)
            .fold(0, |accumulation, item| accumulation + item);
        let weight_sum = payoffs.iter().map(|&(_, count)| count).sum::<usize>();
        weighted_payoff_sum / weight_sum as GeisterPayoff
    }
}

pub struct GeisterComputerStrategy {
    alpha_beta_strategy: AlphaBetaStrategy<GeisterRule, GeisterEvaluator, u32>,
}

impl GeisterComputerStrategy {
    pub fn new(consideration_depth: u32) -> Self {
        Self {
            alpha_beta_strategy: construct_alpha_beta_strategy(consideration_depth),
        }
    }
}

impl Strategy<GeisterState, GeisterAction> for GeisterComputerStrategy {
    fn select_action(&self, state: &GeisterState, actor: Actor) -> Option<GeisterAction> {
        self.alpha_beta_strategy.select_action(state, actor)
    }
}

fn evaluate_payoff_for_exact_state(actor: Actor, exact_state: &GeisterState) -> GeisterPayoff {
    if let Some(winner) = GeisterRule::winner_of(exact_state) {
        if winner == actor {
            100
        } else {
            -100
        }
    } else {
        let mut payoff = 0;
        // フィールドから取り除かれた👻の数で評価する．
        // 自分の邪悪な👻や，相手の善良な👻が取り除かれているほど高評価とする．
        for &a in actors().iter() {
            for &geister in geisters().iter() {
                let coefficient = if a == actor {
                    if geister == Geister::Holy {
                        -1
                    } else {
                        4
                    }
                } else {
                    if geister == Geister::Holy {
                        5
                    } else {
                        -1
                    }
                };
                let killed_count = exact_state.killed_geister_count(OwnedGeister::new(geister, a));
                payoff += coefficient * killed_count as i32;
            }
        }
        // 自分の善良な👻がゴールに近いほど高評価
        {
            let closest_distance = exact_state
                .iterate_owned_geister_positions(OwnedGeister::new(Geister::Holy, actor))
                .map(|p| {
                    let diff = p.try_cast::<isize>().unwrap()
                        - clearable_position_of(actor).try_cast::<isize>().unwrap();
                    let distance = diff.x.abs() + diff.y.abs();
                    distance
                })
                .min()
                .expect("There must be at least 1 holy geister");
            payoff += closest_distance as i32 * 10;
        }
        // 相手の善良な👻がゴールに近いほど低評価ァ！
        {
            let closest_distance = exact_state
                .iterate_owned_geister_positions(OwnedGeister::new(Geister::Holy, actor.opponent()))
                .map(|p| {
                    let diff = p.try_cast::<isize>().unwrap()
                        - clearable_position_of(actor).try_cast::<isize>().unwrap();
                    let distance = diff.x.abs() + diff.y.abs();
                    distance
                })
                .min()
                .expect("There must be at least 1 holy geister");
            payoff -= closest_distance as i32;
        }
        payoff
    }
}

fn iterate_possible_states(
    _viewpoint_actor: Actor,
    state_with_incomplete_info: &GeisterState,
) -> Vec<(GeisterState, usize)> {
    let indexes = vec![1, 2];
    for _combination in indexes.iter().combinations(2) {}
    vec![(state_with_incomplete_info.clone(), 1)]
}
