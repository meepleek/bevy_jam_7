use crate::prelude::{tween::DespawnOnTweenCompleted, *};

#[derive(Default)]
pub struct HideOnStatePlugin<TState: States> {
    pub restore_on_enter_states: Vec<TState>,
    pub restore_on_exit_states: Vec<TState>,
    pub hide_on_enter_states: Vec<TState>,
    pub hide_on_exit_states: Vec<TState>,
}
impl<TState: States> Plugin for HideOnStatePlugin<TState> {
    fn build(&self, app: &mut App) {
        for state in &self.restore_on_enter_states {
            app.add_systems(OnEnter(state.clone()), restore::<TState>);
        }
        for state in &self.restore_on_exit_states {
            app.add_systems(OnExit(state.clone()), restore::<TState>);
        }
        for state in &self.hide_on_enter_states {
            app.add_systems(OnEnter(state.clone()), hide::<TState>);
        }
        for state in &self.hide_on_exit_states {
            app.add_systems(OnExit(state.clone()), hide::<TState>);
        }
    }
}

#[derive(Component)]
struct TweenBackOnStateChange<TState: States>(Vec3, PhantomData<TState>);

#[allow(dead_code)]
pub enum HideTween {
    AbsoluteX(f32),
    RelativeX(f32),
    AbsoluteY(f32),
    RelativeY(f32),
}

// #[allow(dead_code)]
#[derive(Component)]
pub struct HideOnStateChange<TState: States> {
    tween: HideTween,
    despawn: bool,
    _state: PhantomData<TState>,
}
impl<TState: States> HideOnStateChange<TState> {
    pub fn new(tween: HideTween) -> Self {
        Self {
            tween,
            despawn: false,
            _state: PhantomData,
        }
    }

    pub fn with_despawn(mut self) -> Self {
        self.despawn = true;
        self
    }
}

fn hide<TState: States>(
    mut cmd: Commands,
    hide_q: Query<(Entity, &HideOnStateChange<TState>, &Transform)>,
) {
    for (e, hide, hide_t) in hide_q {
        let pos = hide_t.translation;
        let new_pos = match hide.tween {
            HideTween::AbsoluteX(x) => pos.with_x(x),
            HideTween::RelativeX(x) => pos.with_x(pos.x + x),
            HideTween::AbsoluteY(y) => pos.with_y(y),
            HideTween::RelativeY(y) => pos.with_y(pos.y + y),
        };
        let mut e_cmd = or_continue!(cmd.get_entity(e));
        e_cmd.try_insert((
            TweenBackOnStateChange::<TState>(pos, PhantomData::default()),
            tween::get_relative_translation_anim(new_pos.truncate(), 300, None),
        ));
        if hide.despawn {
            e_cmd.try_insert(DespawnOnTweenCompleted::Itself);
        }
    }
}

fn restore<TState: States>(
    mut cmd: Commands,
    restore_q: Query<(Entity, &TweenBackOnStateChange<TState>)>,
) {
    for (e, restore) in restore_q {
        or_continue!(cmd.get_entity(e))
            .try_insert((tween::get_relative_translation_anim(
                restore.0.truncate(),
                300,
                None,
            ),))
            .try_remove::<TweenBackOnStateChange<TState>>();
    }
}
