use crate::GeisterState;
use minimax_strategy::Evaluator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeisterEvaluation(i32);

pub struct GeisterEvaluator {}

impl Evaluator<GeisterState> for GeisterEvaluator {
    type Evaluation = GeisterEvaluation;

    fn evaluate_for_agent(&self, state: &GeisterState) -> Self::Evaluation {
        unimplemented!()
    }
}
