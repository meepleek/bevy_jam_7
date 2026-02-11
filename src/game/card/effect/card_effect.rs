use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum CardEffect {
    TempOffset(i8),
    // Junk,
}
impl CardEffectCommon for CardEffect {
    fn title(&self) -> &str {
        use CardEffect::*;
        match self {
            TempOffset(_) => "Heal self",
        }
    }

    fn temp_offset(&self) -> Option<i8> {
        use CardEffect::*;
        match self {
            TempOffset(offset) => Some(*offset),
        }
    }
}
