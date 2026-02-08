//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input_focus::AutoFocus, prelude::*};

use crate::{
    input::menu::{ButtonClick, LoopingMenu},
    screens::{MainMenuState, PauseState, Screen},
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::MainMenu(MainMenuState::Settings)),
        spawn_settings_menu(Screen::MainMenu(MainMenuState::Settings)),
    )
    .add_systems(
        OnEnter(Screen::Gameplay {
            pause: Some(PauseState::Settings),
        }),
        spawn_settings_menu(Screen::Gameplay {
            pause: Some(PauseState::Settings),
        }),
    );
    app.add_systems(
        Update,
        update_global_volume_label.run_if(in_state(Screen::MainMenu(MainMenuState::Settings))),
    );
}

fn spawn_settings_menu(despawn_state: Screen) -> impl FnMut(Commands) {
    move |mut commands| {
        commands.spawn((
            widget::ui_root("Settings Menu"),
            GlobalZIndex(2),
            LoopingMenu,
            DespawnOnExit(despawn_state),
            children![
                widget::header("Settings"),
                settings_grid(),
                (widget::button("Back", go_back_on_click), AutoFocus),
            ],
        ));
    }
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            global_volume_widget(),
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn go_back_on_click(
    _: On<ButtonClick>,
    screen: Res<State<Screen>>,
    next: ResMut<NextState<Screen>>,
) {
    go_back(screen, next);
}

fn go_back(screen: Res<State<Screen>>, mut next: ResMut<NextState<Screen>>) {
    match screen.get() {
        Screen::MainMenu(main_menu_state) if main_menu_state != &MainMenuState::Title => {
            next.set(Screen::title())
        }
        Screen::Gameplay { pause: Some(_) } => next.set(Screen::paused()),
        _ => warn!("Invalid state to go back from"),
    }
}
