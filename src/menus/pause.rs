use bevy::{input_focus::AutoFocus, prelude::*};

use crate::{
    game::pause::Paused,
    input::menu::{ButtonClick, LoopingMenu, menu_input},
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Paused), (spawn_pause_overlay, spawn_pause_menu));
}

fn pause_overlay() -> impl Bundle {
    (
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Paused),
    )
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn(pause_overlay());
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        LoopingMenu,
        DespawnOnExit(Paused),
        children![
            widget::header("Game paused"),
            (widget::button("Continue", close_menu), AutoFocus),
            widget::button("Settings", open_settings_menu),
            widget::button("Quit to title", quit_to_title),
        ],
        menu_input(),
    ));
}

fn open_settings_menu(_: On<ButtonClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: On<ButtonClick>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::playing());
}

fn quit_to_title(_: On<ButtonClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
