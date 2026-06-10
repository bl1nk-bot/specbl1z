use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TierTag {
    Must,
    Should,
    Avoid,
}

impl From<i32> for TierTag {
    fn from(v: i32) -> Self {
        match v {
            0 => TierTag::Must,
            1 => TierTag::Should,
            2 => TierTag::Avoid,
            _ => TierTag::Must,
        }
    }
}

impl From<TierTag> for i32 {
    fn from(t: TierTag) -> Self {
        match t {
            TierTag::Must => 0,
            TierTag::Should => 1,
            TierTag::Avoid => 2,
        }
    }
}

impl rusqlite::ToSql for TierTag {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(i32::from(self.clone())))
    }
}

impl rusqlite::types::FromSql for TierTag {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|i| TierTag::from(i as i32))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub key: String,
    pub label: String,
    pub icon: String,
    pub order_index: i32,
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier {
    pub id: i32,
    pub section_id: i32,
    pub text: String,
    pub tag: TierTag,
    pub code: Option<String>,
    pub order_index: i32,
    pub is_custom: bool,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProgress {
    pub user_id: String,
    pub tier_id: i32,
    pub checked: bool,
}

// ==================== Memory System ====================

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MemoryScope {
    ScopeGlobal = 0,
    ScopeProject = 1,
    ScopeSession = 2,
    ScopeWorking = 3,
    ScopePolicy = 4,
    ScopeIdentity = 5,
}

impl TryFrom<i32> for MemoryScope {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MemoryScope::ScopeGlobal),
            1 => Ok(MemoryScope::ScopeProject),
            2 => Ok(MemoryScope::ScopeSession),
            3 => Ok(MemoryScope::ScopeWorking),
            4 => Ok(MemoryScope::ScopePolicy),
            5 => Ok(MemoryScope::ScopeIdentity),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MemoryCategory {
    CategoryFact = 0,
    CategoryPreference = 1,
    CategoryHistory = 2,
    CategoryContext = 3,
    CategoryInference = 4,
}

impl TryFrom<i32> for MemoryCategory {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MemoryCategory::CategoryFact),
            1 => Ok(MemoryCategory::CategoryPreference),
            2 => Ok(MemoryCategory::CategoryHistory),
            3 => Ok(MemoryCategory::CategoryContext),
            4 => Ok(MemoryCategory::CategoryInference),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTopic {
    TopicLearn = 0,
    TopicWork = 1,
    TopicTool = 2,
    TopicInterest = 3,
    TopicProject = 4,
    TopicIdentify = 5,
}

impl TryFrom<i32> for MemoryTopic {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MemoryTopic::TopicLearn),
            1 => Ok(MemoryTopic::TopicWork),
            2 => Ok(MemoryTopic::TopicTool),
            3 => Ok(MemoryTopic::TopicInterest),
            4 => Ok(MemoryTopic::TopicProject),
            5 => Ok(MemoryTopic::TopicIdentify),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryEntry {
    pub id: Option<i64>,
    pub scope: i32,
    pub category: i32,
    pub topic: i32,
    pub key: String,
    pub value: String,
    pub source: Option<String>,
    pub confidence: f32,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: i32,
    pub status: String,
    pub tags: Vec<String>,
    pub owner: Option<String>,
    pub access_level: String,
    pub provenance: Option<String>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryLink {
    pub id: i64,
    pub from_entry_id: i64,
    pub to_entry_id: i64,
    pub relation_type: String,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryAuditLog {
    pub id: i64,
    pub entry_id: i64,
    pub operation: String,
    pub performed_by: String,
    pub timestamp: i64,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

// ==================== Prompt System ====================

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PromptBlockType {
    BlockSystem = 0,
    BlockPolicy = 1,
    BlockIdentity = 2,
    BlockMemory = 3,
    BlockTask = 4,
    BlockToolInstruction = 5,
    BlockPlan = 6,
    BlockUserInput = 7,
    BlockGuardrail = 8,
    BlockOutputFormat = 9,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptBlock {
    pub id: String,
    pub block_type: i32,
    pub priority: i32,
    pub scope: i32,
    pub content: String,
    pub source: String,
    pub constraints: Option<String>,
    pub dependencies: Vec<String>,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextPack {
    pub session_id: String,
    pub blocks: Vec<PromptBlock>,
    pub assembled_at: i64,
    pub trace_id: Option<String>,
    pub total_tokens_estimate: i32,
}
