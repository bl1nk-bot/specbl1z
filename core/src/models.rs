use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleTag {
    Must,
    Should,
    Avoid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub key: String,
    pub label: String,
    pub icon: String,
    pub order_index: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub icon: String,
    pub color: String,
    pub text_color: String,
    pub order_index: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub id: i32,
    pub section_id: i32,
    pub text: String,
    pub tag: RuleTag,
    pub code: Option<String>,
    pub order_index: i32,
    pub is_custom: bool,
    pub user_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProgress {
    pub user_id: String,
    pub rule_id: i32,
    pub checked: bool,
    pub updated_at: Option<DateTime<Utc>>,
}
