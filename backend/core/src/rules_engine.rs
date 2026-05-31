use crate::models::{Tier, TierTag};

pub struct TierProcessor;

impl TierProcessor {
    /// ตรวจสอบความถูกต้องของกฎตามมาตรฐานที่กำหนด
    pub fn validate_tier(tier: &Tier) -> Result<bool, String> {
        if tier.text.is_empty() {
            return Err("Tier text cannot be empty".to_string());
        }

        // ตัวอย่าง Logic เฉพาะของ specgen: ตรวจสอบความยาวหรือรูปแบบ
        if tier.tag == TierTag::Must && tier.text.len() < 10 {
            return Err("Critical tiers (MUST) must have detailed description".to_string());
        }

        Ok(true)
    }

    /// รวมกฎจากหลายแหล่งเข้าด้วยกัน (Deduplication Logic)
    pub fn merge_tiers(existing: Vec<Tier>, new_tiers: Vec<Tier>) -> Vec<Tier> {
        let mut merged = existing;
        for new_t in new_tiers {
            if !merged.iter().any(|t| t.text == new_t.text) {
                merged.push(new_t);
            }
        }
        merged
    }
}
