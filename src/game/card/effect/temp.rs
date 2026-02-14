use bevy_trauma_shake::Shakes;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Temp>().add_observer(handle_temp_change);
}

#[derive(Resource, Debug)]
pub struct Temp {
    pub current: u8,
    pub max: u8,
}
impl Temp {
    pub const fn default_max() -> u8 {
        10
    }

    pub const fn default_initial() -> u8 {
        Self::default_max().div_ceil(2)
    }

    pub fn ratio(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub fn fill_color(&self) -> Color {
        let third = self.max / 3;
        if self.current < third {
            COL_BLUE
        } else if self.current > third * 2 {
            COL_RED
        } else {
            COL_ORANGE
        }
    }

    /// Updates health saturating it at bounds and returning whether it has overlown
    #[must_use]
    pub fn update_current(&mut self, change: i8) -> bool {
        let unclamped = self.current.saturating_add_signed(change);
        let taken_dmg = unclamped == 0 || unclamped > self.max;
        self.current = unclamped.clamp(1, self.max);
        taken_dmg
    }
}

impl Default for Temp {
    fn default() -> Self {
        Self {
            current: Self::default_initial(),
            max: Self::default_max(),
        }
    }
}

#[derive(Event, Debug)]
pub struct TempChangeAction {
    pub change: i8,
}

fn handle_temp_change(action: On<TempChangeAction>, mut temp: ResMut<Temp>, mut shake: Shakes) {
    tracing::info!(?temp, change = action.change, "Updating temp");
    shake.add_trauma(0.1 * action.change.abs() as f32);

    let take_dmg = temp.update_current(action.change);
    if take_dmg {
        // todo: figure out what makes the game more nailbiting
        // a: just temp
        // b: HP which is damaged on temp overflow
        tracing::warn!("impl taking dmg or maybe just dying");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    #[test_case(3, 1 => (4, false))]
    #[test_case(3, 2 => (5, false))]
    #[test_case(4, 2 => (5, true))]
    #[test_case(3, -2 => (1, false))]
    #[test_case(2, -2 => (1, true))]
    #[test_case(2, -3 => (1, true))]
    #[traced_test]
    fn temp_update_current(current: u8, change: i8) -> (u8, bool) {
        let mut temp = Temp { current, max: 5 };
        let take_dmg = temp.update_current(change);
        (temp.current, take_dmg)
    }
}
