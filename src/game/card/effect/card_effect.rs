use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum CardEffect {
    TempOffset(i8),
    // Junk,
}
impl CardEffectCommon for CardEffect {
    fn temp_offset(&self) -> Option<i8> {
        use CardEffect::*;
        match self {
            TempOffset(offset) => Some(*offset),
        }
    }

    fn effect_palette(&self) -> CardEffectPalette {
        match self {
            CardEffect::TempOffset(offset) => {
                if *offset > 0 {
                    CardEffectPalette::single_color(COL_CARD_TEMP_UP)
                } else {
                    CardEffectPalette::single_color(COL_CARD_TEMP_DOWN)
                }
            }
        }
    }
}
