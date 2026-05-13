use crate::bl1nk::{Rule, RuleTag};

pub struct RuleProcessor;

impl RuleProcessor {
    /// ตรวจสอบความถูกต้องของกฎตามมาตรฐานที่กำหนด
    pub fn validate_rule(rule: &Rule) -> Result<bool, String> {
        if rule.text.is_empty() {
            return Err("Rule text cannot be empty".to_string());
        }

        // ตัวอย่าง Logic เฉพาะของ specgen: ตรวจสอบความยาวหรือรูปแบบ
        if rule.tag == RuleTag::Must as i32 && rule.text.len() < 10 {
            return Err("Critical rules (MUST) must have detailed description".to_string());
        }

        Ok(true)
    }

    /// รวมกฎจากหลายแหล่งเข้าด้วยกัน (Deduplication Logic)
    pub fn merge_rules(existing: Vec<Rule>, new_rules: Vec<Rule>) -> Vec<Rule> {
        let mut merged = existing;
        for new_r in new_rules {
            if !merged.iter().any(|r| r.text == new_r.text) {
                merged.push(new_r);
            }
        }
        merged
    }
}
