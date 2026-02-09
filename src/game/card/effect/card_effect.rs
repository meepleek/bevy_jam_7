use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum CardEffect {
    HealSelf(u8),
    // Junk,
}
impl CardEffectCommon for CardEffect {
    fn title(&self) -> &str {
        use CardEffect::*;
        match self {
            HealSelf(_) => "Heal self",
        }
    }

    fn temp_offset(&self) -> Option<i8> {
        use CardEffect::*;
        match self {
            HealSelf(heal) => Some(*heal as i8),
        }
    }
}
