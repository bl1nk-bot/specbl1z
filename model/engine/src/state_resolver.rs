use crate::label_parser::NormalizedState;

pub fn resolve_state(state: &mut NormalizedState) {
    // Basic resolution logic
    if let Some(s) = &state.state {
        if s != "backlog" && state.agent.is_none() {
            // If state is not backlog but no agent is assigned,
            // we might want to flag it or assign a default.
            // For now, we'll just leave it and let the consumer handle it.
        }
    }

    // Default status if state exists
    if state.state.is_some() && state.status.is_none() {
        state.status = Some("in-progress".to_string());
    }
}
