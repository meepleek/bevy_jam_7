//! The main menu (seen on the title screen).

use bevy::{input_focus::AutoFocus, prelude::*};

use crate::{
    input::menu::{ButtonClick, LoopingMenu, menu_input},
    screens::{MainMenuState, Screen},
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::title()), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        LoopingMenu,
        DespawnOnExit(Screen::title()),
        #[cfg(not(target_family = "wasm"))]
        children![
            (widget::button("Play", enter_gameplay_screen), AutoFocus),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
        menu_input(),
    ));
}

fn enter_gameplay_screen(_: On<ButtonClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::playing());
}

fn open_settings_menu(_: On<ButtonClick>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::MainMenu(MainMenuState::Settings));
}

fn open_credits_menu(_: On<ButtonClick>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::MainMenu(MainMenuState::Credits));
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<ButtonClick>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
