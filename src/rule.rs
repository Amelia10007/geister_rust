use crate::{GeisterAction, GeisterMovement, GeisterState};
use minimax_strategy::{Actor, Rule};

pub struct GeisterRule {}

impl Rule<GeisterState, GeisterAction> for GeisterRule {
    type ActionIterator = Vec<GeisterAction>;

    fn iterate_available_actions(
        &self,
        state: &GeisterState,
        actor: Actor,
    ) -> Self::ActionIterator {
        unimplemented!()
    }

    fn translate_state(&self, state: &GeisterState, action: &GeisterAction) -> GeisterState {
        unimplemented!()
    }
}
