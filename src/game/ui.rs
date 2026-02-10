use bevy::{color::palettes::css::YELLOW, ui::InteractionDisabled};

use crate::{game::turn::TurnOrder, input::menu::ButtonClick, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayPhase::LevelSpawn), spawn_ui)
        .add_systems(OnEnter(TurnOrder::Ai), disable_end_turn_button)
        .add_systems(
            Update,
            handle_temp_change.run_if(resource_exists_and_changed::<Temp>),
        );
}

#[derive(Component)]
struct EndTurnButton;

#[derive(Component)]
struct TempFill;
impl TempFill {
    pub fn perc_val(current: u8, max: u8) -> Val {
        percent((100. / max as f32) * current as f32)
    }
}

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
            padding: UiRect::all(px(30)),
            grid_template_rows: vec![
                GridTrack::min_content(),
                GridTrack::fr(1.0),
                GridTrack::min_content(),
            ],
            grid_template_columns: vec![
                GridTrack::min_content(),
                GridTrack::fr(1.0),
                GridTrack::min_content(),
            ],
            ..default()
        },
        children![
            temp_bar(),
            (
                Node {
                    justify_self: JustifySelf::End,
                    grid_column: GridPlacement::start_span(2, 2),
                    ..default()
                },
                children![(widget::button("End turn", end_turn), EndTurnButton),]
            ),
            Node {
                justify_self: JustifySelf::Start,
                grid_column: GridPlacement::start_span(2, 2),
                ..default()
            },
            Node {
                justify_self: JustifySelf::Start,
                grid_column: GridPlacement::start_span(2, 2),
                ..default()
            },
        ],
    )
}

fn temp_bar() -> impl Bundle {
    (
        Node {
            flex_basis: percent(100),
            align_self: AlignSelf::Stretch,
            padding: UiRect::all(px(10)),
            grid_row: GridPlacement::span(3),
            ..default()
        },
        BackgroundColor(YELLOW.into()),
        children![(
            Node {
                align_items: AlignItems::End,
                width: px(50),
                height: percent(100),
                padding: UiRect::all(px(4)),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            children![(
                Node {
                    height: TempFill::perc_val(Temp::default_initial(), Temp::default_max()),
                    width: percent(100.),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                TempFill,
            )]
        ),],
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

fn handle_temp_change(temp: Res<Temp>, mut node: Single<&mut Node, With<TempFill>>) {
    node.height = TempFill::perc_val(temp.current, temp.max);
}
