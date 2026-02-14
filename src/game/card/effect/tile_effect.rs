use crate::prelude::attack::AttackAction;
pub use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(play_selected_tile_card)
        .add_observer(process_selected_tile_trigger_card);
}

#[derive(Debug, Clone)]
pub struct EffectTarget {
    pub reach: EffectReach,
    pub direction: EffectDirection,
}
impl EffectTarget {
    pub fn target_tiles(&self) -> Vec<Coords> {
        let range = match self.reach {
            EffectReach::Exact(val) => val as i16..=val as i16,
            EffectReach::Range(max) => 1..=max as i16,
        };
        match self.direction {
            EffectDirection::Area => match self.reach {
                EffectReach::Exact(val) => {
                    let val = val as i16;
                    let range = -val..=val;
                    let mut res = HashSet::with_capacity(val as usize * 2 * 5);
                    res.extend(
                        range
                            .clone()
                            .flat_map(|x| [-val, val].map(|y| Coords::new(x, y))),
                    );
                    res.extend(range.flat_map(|y| [-val, val].map(|x| Coords::new(x, y))));
                    res.into_iter().collect()
                }
                EffectReach::Range(max) => {
                    let max = max as i16;
                    let range = -max..=max;
                    range
                        .clone()
                        .flat_map(|y| range.clone().map(move |x| (x, y).into()))
                        .filter(|tile| tile != &Coords::ZERO)
                        .collect()
                }
            },
            EffectDirection::Orthogonal => range
                .flat_map(|i| {
                    [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .map(|(sign_x, sign_y)| Coords::new(sign_x, sign_y) * i)
                })
                .collect(),
            EffectDirection::Diagonal => range
                .flat_map(|i| {
                    [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                        .map(|(sign_x, sign_y)| Coords::new(sign_x, sign_y) * i)
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TileCardEffect {
    Move {
        target: EffectTarget,
        temp_offset: i8,
    },
    Attack {
        target: EffectTarget,
        temp_offset: i8,
    },
}
impl CardEffectCommon for TileCardEffect {
    fn temp_offset(&self) -> Option<i8> {
        use TileCardEffect::*;
        match self {
            Move { temp_offset, .. } | Attack { temp_offset, .. } => Some(-(*temp_offset)),
        }
    }

    fn effect_palette(&self) -> CardEffectPalette {
        match self {
            TileCardEffect::Move { .. } => {
                CardEffectPalette::new(COL_CARD_EFFECT_MOVEMENT, COL_CARD_EFFECT_MOVEMENT_HOVER)
            }
            TileCardEffect::Attack { .. } => {
                CardEffectPalette::new(COL_CARD_EFFECT_ATTACK, COL_CARD_EFFECT_ATTACK_HOVER)
            }
        }
    }
}
impl TileCardEffect {
    pub fn tile_target(&self) -> TileTarget {
        use TileCardEffect::*;

        match self {
            Move { .. } => TileTarget::Empty,
            Attack { .. } => TileTarget::Enemy,
        }
    }

    pub fn tiles(&self) -> Vec<Coords> {
        use TileCardEffect::*;
        match self {
            Move { target, .. } | Attack { target, .. } => target.target_tiles(),
        }
    }
}

fn play_selected_tile_card(
    event: On<PlaySelectedTileCard>,
    player: Single<Entity, With<Player>>,
    // discard_pile: Single<Entity, With<DiscardPile>>,
    card_q: Query<&Card>,
    mut cmd: Commands,
    mut cards: Cards,
) {
    use TileCardEffect::*;
    let card = or_return!(card_q.get(event.card_e));
    match &card.effect_trigger {
        CardEffectTrigger::TileSelection(tile_card_action) => match tile_card_action {
            Move { temp_offset, .. } => {
                cmd.trigger(MoveAction {
                    agent_e: *player,
                    to: event.selected_tile,
                });
                cmd.trigger(TempChangeAction {
                    change: *temp_offset,
                });
            }
            Attack { temp_offset, .. } => {
                cmd.trigger(TempChangeAction {
                    change: *temp_offset,
                });
                cmd.trigger(AttackAction {
                    tile: event.selected_tile,
                });
            }
        },
        CardEffectTrigger::CardSelection(_) => {
            error!(?card, "Card should not have been played on tile selection");
            unreachable!();
        }
    }
    cards.discard_card(event.card_e);
}

fn process_selected_tile_trigger_card(
    trig: On<Add, SelectedTileTriggerCard>,
    card_q: Query<&Card>,
    mut cmd: Commands,
    grid: Single<&Grid>,
    player: Single<Entity, With<Player>>,
) {
    let card = or_return!(card_q.get(trig.event_target()));
    let player_tile = or_return!(grid.entity_to_coords(*player));
    match &card.effect_trigger {
        CardEffectTrigger::TileSelection(action) => {
            let card_e = trig.event_target();
            let effect_palette = card.effect_trigger.effect_palette();
            for tile in action.tiles().into_iter().map(|tile| player_tile + tile) {
                let tile_obj = grid.get_tile_object(tile);
                let targetable = match action.tile_target() {
                    TileTarget::Empty => tile_obj.is_none(),
                    TileTarget::Enemy => match tile_obj {
                        Some(TileObject {
                            kind: TileObjectKind::Enemy,
                            ..
                        }) => true,
                        _ => false,
                    },
                };
                if !targetable {
                    continue;
                }

                let tile_e = or_continue!(grid.get_tile_entity(tile));
                or_continue!(cmd.get_entity(tile_e))
                    .insert((
                        tween::get_relative_sprite_color_anim(
                            effect_palette.highlighted,
                            150,
                            None,
                        ),
                        TileHighlighted,
                    ))
                    .observe(tween::tween_sprite_color_on_trigger::<Pointer<Over>, ()>(
                        effect_palette.hover,
                    ))
                    .observe(tween::tween_sprite_color_on_trigger::<Pointer<Out>, ()>(
                        effect_palette.highlighted,
                    ))
                    .observe(move |_ev: On<Pointer<Click>>, mut cmd: Commands| {
                        cmd.trigger(PlaySelectedTileCard {
                            card_e,
                            selected_tile: tile,
                        });
                    });
            }
        }
        CardEffectTrigger::CardSelection(_) => {
            unreachable!("Card should have been played directly")
        }
    }
}
