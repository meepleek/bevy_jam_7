use bevy::ui::InteractionDisabled;

use crate::{game::turn::TurnOrder, input::menu::ButtonClick, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayPhase::LevelSpawn), spawn_ui)
        .add_systems(OnEnter(TurnOrder::Ai), disable_end_turn_button);
}

#[derive(Component)]
struct EndTurnButton;

fn ui() -> impl Bundle {
    (
        Name::new("UI Grid"),
        Pickable::IGNORE,
        Node {
            display: Display::Grid,
            row_gap: px(30),
            column_gap: px(30),
            width: percent(100),
            height: percent(100),
            grid_template_rows: vec![
                GridTrack::flex(1.0),
                GridTrack::fr(1.0),
                GridTrack::flex(1.0),
            ],
            ..default()
        },
        children![
            (
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
                children![(widget::button("End turn", end_turn), EndTurnButton),]
            ),
            Node {
                justify_self: JustifySelf::Start,
                ..default()
            },
            Node {
                justify_self: JustifySelf::Start,
                ..default()
            },
        ],
    )
}

fn spawn_ui(mut cmd: Commands) {
    cmd.spawn(ui());
}

fn end_turn(_ev: On<ButtonClick>, mut turn: ResMut<NextState<TurnOrder>>) {
    turn.set(TurnOrder::Ai);
}

fn disable_end_turn_button(btn: Single<Entity, With<EndTurnButton>>, mut cmd: Commands) {
    // todo: actually properly disable the button
    or_return!(cmd.get_entity(*btn)).insert(InteractionDisabled);
}
