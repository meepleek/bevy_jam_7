use crate::prelude::*;

pub mod attack;
pub mod card_effect;
pub mod temp;
pub mod tile_effect;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((tile_effect::plugin, temp::plugin, attack::plugin))
        .add_observer(play_card);
}

// #[allow(dead_code)]
// #[derive(Component)]
// pub struct ActionQueue {
//     // todo:
//     actions: VecDeque<()>,
// }

pub enum TileTarget {
    Empty,
    Enemy,
}

#[derive(Debug, Clone)]
pub enum CardEffectTrigger {
    CardSelection(CardEffect),
    TileSelection(TileCardEffect),
}
impl CardEffectCommon for CardEffectTrigger {
    fn title(&self) -> &str {
        match self {
            CardEffectTrigger::CardSelection(action) => action.title(),
            CardEffectTrigger::TileSelection(action) => action.title(),
        }
    }

    fn temp_offset(&self) -> Option<i8> {
        match self {
            CardEffectTrigger::CardSelection(action) => action.temp_offset(),
            CardEffectTrigger::TileSelection(action) => action.temp_offset(),
        }
    }
}

pub trait CardEffectCommon {
    fn title(&self) -> &str;
    fn temp_offset(&self) -> Option<i8>;
    // todo: kind
    // like action, passive, timed passive?
}

#[derive(Debug, Clone)]
pub struct TileInteractionPalette {
    pub highlight: Color,
    pub hover: Color,
}
impl TileInteractionPalette {
    pub fn new(highlight: impl Into<Color>, hover: impl Into<Color>) -> Self {
        Self {
            highlight: highlight.into(),
            hover: hover.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EffectDirection {
    #[allow(dead_code)]
    Area,
    Orthogonal,
    #[allow(dead_code)]
    Diagonal,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectReach {
    Exact(u8),
    Range(u8),
}

// #[allow(dead_code)]
// #[derive(Debug, Clone)]
// pub enum CardActionCondition {
//     Temp(RangeInclusive<u8>),
// }

#[derive(Event)]
pub struct PlayCard(pub Entity);

fn play_card(ev: On<PlayCard>, card_q: Query<&Card>, mut cmd: Commands, mut cards: Cards) {
    use CardEffect::*;
    let card = or_return!(card_q.get(ev.0));
    match &card.effect_trigger {
        CardEffectTrigger::CardSelection(action) => match action {
            TempOffset(offset) => cmd.trigger(TempChangeAction { change: *offset }),
        },
        CardEffectTrigger::TileSelection(_) => {
            error!(?card, "Card should not have been played on selection");
            unreachable!();
        }
    }
    cards.discard_card(ev.0);
}

#[derive(Event)]
pub struct PlaySelectedTileCard {
    pub card_e: Entity,
    pub selected_tile: Coords,
}
