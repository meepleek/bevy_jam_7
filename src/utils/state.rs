use crate::prelude::{tween::DespawnOnTweenCompleted, *};

#[derive(Default)]
pub struct HideOnStatePlugin<TState: States>(PhantomData<TState>);
impl<TState: States> Plugin for HideOnStatePlugin<TState> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, restore::<TState>.run_if(state_changed::<TState>));
        app.add_systems(Update, hide::<TState>.run_if(state_changed::<TState>));
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
    restore_on_enter_states: Vec<TState>,
    restore_on_exit_states: Vec<TState>,
    hide_on_enter_states: Vec<TState>,
    hide_on_exit_states: Vec<TState>,
}
#[allow(dead_code)]
impl<TState: States> HideOnStateChange<TState> {
    pub fn restore_on_enter(tween: HideTween, state: TState) -> Self {
        let mut hide = Self::base(tween);
        hide.restore_on_enter_states = vec![state];
        hide
    }

    pub fn with_restore_on_enter(mut self, state: TState) -> Self {
        self.restore_on_enter_states.push(state);
        self
    }

    pub fn restore_on_exit(tween: HideTween, state: TState) -> Self {
        let mut hide = Self::base(tween);
        hide.restore_on_enter_states = vec![state];
        hide
    }

    pub fn with_restore_on_exit(mut self, state: TState) -> Self {
        self.restore_on_exit_states.push(state);
        self
    }

    pub fn hide_on_enter(tween: HideTween, state: TState) -> Self {
        let mut hide = Self::base(tween);
        hide.hide_on_enter_states = vec![state];
        hide
    }

    pub fn with_hide_on_enter(mut self, state: TState) -> Self {
        self.hide_on_enter_states.push(state);
        self
    }

    pub fn hide_on_exit(tween: HideTween, state: TState) -> Self {
        let mut hide = Self::base(tween);
        hide.hide_on_exit_states = vec![state];
        hide
    }

    pub fn with_hide_on_exit(mut self, state: TState) -> Self {
        self.hide_on_exit_states.push(state);
        self
    }

    pub fn with_despawn(mut self) -> Self {
        self.despawn = true;
        self
    }

    fn base(tween: HideTween) -> Self {
        Self {
            tween,
            despawn: false,
            restore_on_enter_states: Vec::default(),
            restore_on_exit_states: Vec::default(),
            hide_on_enter_states: Vec::default(),
            hide_on_exit_states: Vec::default(),
        }
    }
}

fn hide<TState: States>(
    mut cmd: Commands,
    hide_q: Query<(Entity, &HideOnStateChange<TState>, &Transform)>,
    mut state_reader: MessageReader<StateTransitionEvent<TState>>,
) {
    let msgs: Vec<_> = state_reader.read().collect();
    if msgs.is_empty() {
        return;
    }

    for (e, hide, hide_t) in hide_q {
        if !msgs.iter().any(|msg| {
            if let Some(entered) = msg.entered.as_ref()
                && hide.hide_on_enter_states.contains(entered)
            {
                return true;
            }
            if let Some(exited) = msg.exited.as_ref()
                && hide.hide_on_exit_states.contains(exited)
            {
                return true;
            }

            false
        }) {
            return;
        }

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
    restore_q: Query<(
        Entity,
        &HideOnStateChange<TState>,
        &TweenBackOnStateChange<TState>,
    )>,
    mut state_reader: MessageReader<StateTransitionEvent<TState>>,
) {
    let msgs: Vec<_> = state_reader.read().collect();
    if msgs.is_empty() {
        return;
    }

    for (e, hide, restore) in restore_q {
        if !msgs.iter().any(|msg| {
            if let Some(entered) = msg.entered.as_ref()
                && hide.restore_on_enter_states.contains(entered)
            {
                return true;
            }
            if let Some(exited) = msg.exited.as_ref()
                && hide.restore_on_exit_states.contains(exited)
            {
                return true;
            }

            false
        }) {
            return;
        }

        or_continue!(cmd.get_entity(e))
            .try_insert((tween::get_relative_translation_anim(
                restore.0.truncate(),
                300,
                None,
            ),))
            .try_remove::<TweenBackOnStateChange<TState>>();
    }
}
