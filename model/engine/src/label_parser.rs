use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug, Clone)]
pub struct NormalizedState {
    #[pyo3(get)]
    pub agent: Option<String>,
    #[pyo3(get)]
    pub state: Option<String>,
    #[pyo3(get)]
    pub task_type: Option<String>,
    #[pyo3(get)]
    pub priority: Option<String>,
    #[pyo3(get)]
    pub platform: Option<String>,
    #[pyo3(get)]
    pub env: Option<String>,
    #[pyo3(get)]
    pub status: Option<String>,
    #[pyo3(get)]
    pub extra: HashMap<String, String>,
}

#[pymethods]
impl NormalizedState {
    #[new]
    fn new() -> Self {
        Self {
            agent: None,
            state: None,
            task_type: None,
            priority: None,
            platform: None,
            env: None,
            status: None,
            extra: HashMap::new(),
        }
    }
}

pub fn normalize_labels(labels: Vec<String>) -> NormalizedState {
    let mut state = NormalizedState::new();

    for label in labels {
        if let Some((prefix, value)) = label.split_once(':') {
            let value = value.trim().to_string();
            match prefix.trim() {
                "agent" => state.agent = Some(value),
                "state" => state.state = Some(value),
                "type" => state.task_type = Some(value),
                "p" => state.priority = Some(value),
                "plat" => state.platform = Some(value),
                "env" => state.env = Some(value),
                "status" => state.status = Some(value),
                _ => {
                    state.extra.insert(prefix.trim().to_string(), value);
                }
            }
        }
    }

    state
}
