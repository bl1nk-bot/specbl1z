use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    #[pyo3(get)]
    pub event_type: String,
    #[pyo3(get)]
    pub labels: Vec<String>,
    #[pyo3(get)]
    pub actor: String,
    #[pyo3(get)]
    pub timestamp: String,
    #[pyo3(get)]
    pub metadata: HashMap<String, String>,
}

#[pymethods]
impl TimelineEvent {
    #[new]
    fn new(event_type: String, labels: Vec<String>, actor: String, timestamp: String) -> Self {
        Self {
            event_type,
            labels,
            actor,
            timestamp,
            metadata: HashMap::new(),
        }
    }
}

pub fn build_timeline_event(
    event_type: String,
    labels: Vec<String>,
    actor: String,
    timestamp: String,
) -> TimelineEvent {
    TimelineEvent::new(event_type, labels, actor, timestamp)
}
