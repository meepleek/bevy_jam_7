#![allow(dead_code)]

#[cfg(feature = "dev")]
use bevy::dev_tools::states::log_transitions;

use crate::{prelude::*, utils::state::HideOnStatePlugin};
use bevy::{ecs::system::SystemParam, state::state::FreelyMutableState};

#[derive(Default)]
pub struct FadeStatePlugin<TState: States + FreelyMutableState>(PhantomData<TState>);
impl<TState: States + FreelyMutableState> Plugin for FadeStatePlugin<TState> {
    fn build(&self, app: &mut App) {
        app.init_state::<FadeState<TState>>()
            .init_resource::<NextFadeState<TState>>()
            .add_systems(Update, tick_fade_timer::<TState>)
            .add_plugins(HideOnStatePlugin::<FadeState<TState>>::default());

        #[cfg(feature = "dev")]
        app.add_systems(Last, log_transitions::<FadeState<TState>>);
    }
}

#[allow(nonstandard_style)]
pub fn OnEnterFadingFrom<TState: States>(state: TState) -> OnEnter<FadeState<TState>> {
    OnEnter(FadeState::FadingOut(state))
}
#[allow(nonstandard_style)]
pub fn OnExitFadingFrom<TState: States>(state: TState) -> OnExit<FadeState<TState>> {
    OnExit(FadeState::FadingOut(state))
}
#[allow(nonstandard_style)]
pub fn OnEnterFadingTo<TState: States>(state: TState) -> OnEnter<FadeState<TState>> {
    OnEnter(FadeState::FadingIn(state))
}
#[allow(nonstandard_style)]
pub fn OnExitFadingTo<TState: States>(state: TState) -> OnExit<FadeState<TState>> {
    OnExit(FadeState::FadingIn(state))
}

pub fn is_fading_from<TState: States>(
    state: TState,
) -> impl FnMut(Option<Res<State<FadeState<TState>>>>) -> bool + Clone {
    move |fade_state| match fade_state {
        Some(fade_state) => match fade_state.get() {
            FadeState::FadingOut(from_state) => *from_state == state,
            _ => false,
        },
        None => false,
    }
}

pub fn is_fading_to<TState: States>(
    state: TState,
) -> impl FnMut(Option<Res<State<FadeState<TState>>>>) -> bool + Clone {
    move |fade_state| match fade_state {
        Some(fade_state) => match fade_state.get() {
            FadeState::FadingIn(to_state) => *to_state == state,
            _ => false,
        },
        None => false,
    }
}

#[derive(SystemParam)]
pub struct FadeStates<'w, /* 's, */ TState: States + FreelyMutableState> {
    from: Res<'w, State<TState>>,
    to: ResMut<'w, NextState<TState>>,
    next_fade: ResMut<'w, NextState<FadeState<TState>>>,
    next_fade_state: ResMut<'w, NextFadeState<TState>>,
}
impl<'w, /* 's, */ TState: States + FreelyMutableState> FadeStates<'w, /* 's, */ TState> {
    pub fn set_with_delay(&mut self, state: TState, in_ms: u64, out_ms: u64) {
        let from = self.from.get().clone();
        self.next_fade_state.next = Some(FadeStateData {
            from: from.clone(),
            to: state,
            in_timer: Some(Timer::new(Duration::from_millis(in_ms), TimerMode::Once)),
            out_timer: Timer::new(Duration::from_millis(out_ms), TimerMode::Once),
        });
        self.next_fade.set(FadeState::FadingOut(from));
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum FadeState<TState: States> {
    #[default]
    NotFading,
    FadingOut(TState),
    FadingIn(TState),
}

struct FadeStateData<TState: States> {
    from: TState,
    to: TState,
    in_timer: Option<Timer>,
    out_timer: Timer,
}

#[derive(Resource)]
pub struct NextFadeState<TState: States> {
    next: Option<FadeStateData<TState>>,
}
impl<TState: States + FreelyMutableState> NextFadeState<TState> {
    pub fn set_with_delay(&mut self, from: TState, to: TState, in_ms: u64, out_ms: u64) {
        self.next = Some(FadeStateData {
            from,
            to,
            in_timer: Some(Timer::new(Duration::from_millis(in_ms), TimerMode::Once)),
            out_timer: Timer::new(Duration::from_millis(out_ms), TimerMode::Once),
        });
    }
}
impl<TState: States + FreelyMutableState> Default for NextFadeState<TState> {
    fn default() -> Self {
        Self { next: None }
    }
}

fn tick_fade_timer<TState: States + FreelyMutableState>(
    mut next_fade_state: ResMut<NextFadeState<TState>>,
    time: Res<Time>,
    mut next: ResMut<NextState<TState>>,
    mut next_fade: ResMut<NextState<FadeState<TState>>>,
    current_next_fade: Res<State<FadeState<TState>>>,
) {
    if let Some(nfd) = next_fade_state.next.as_mut() {
        if current_next_fade.get() == &FadeState::<TState>::NotFading {
            next_fade.set(FadeState::FadingOut(nfd.from.clone()));
        }
        if let Some(timer) = nfd.in_timer.as_mut() {
            timer.tick(time.delta());
            if timer.just_finished() {
                // clear_in = true;
                _ = nfd.in_timer.take();
                next_fade.set(FadeState::FadingIn(nfd.to.clone()));
                next.set(nfd.to.clone());
            }
        } else {
            nfd.out_timer.tick(time.delta());
            if nfd.out_timer.just_finished() {
                _ = next_fade_state.next.take();
                next_fade.set(FadeState::NotFading);
            }
        }
    }
}
