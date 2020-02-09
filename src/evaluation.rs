use crate::GeisterState;
use minimax_strategy::{Actor, Evaluator};

/// ゲームGeisterのフィールドに対する利得を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeisterPayoff(i32);

/// ゲームGeisterのフィールドに対する評価関数を表す．
pub struct GeisterEvaluator {}

impl Evaluator<GeisterState> for GeisterEvaluator {
    type Payoff = GeisterPayoff;

    fn evaluate_payoff_for(_actor: Actor, _state: &GeisterState) -> Self::Payoff {
        unimplemented!()
    }
}
