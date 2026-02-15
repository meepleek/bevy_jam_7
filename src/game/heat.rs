use bevy::math::U16Vec2;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Heat>();
}

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct Heat(pub u8);
impl Default for Heat {
    fn default() -> Self {
        Self(1)
    }
}
impl Heat {
    pub fn target_kill_count(&self) -> usize {
        let base = 3;
        let increase = 2;
        base + self.0 as usize * increase
    }

    pub fn enemy_max(&self) -> usize {
        let base = 4;
        base + self.0 as usize / 2
    }

    pub fn grid_size(&self) -> U16Vec2 {
        match self.0 {
            ..=2 => (7, 7).into(),
            3..=4 => (7, 5).into(),
            _ => (5, 5).into(),
        }
    }

    #[expect(dead_code)]
    pub fn obstacle_max(&self) -> usize {
        let base = 3;
        let increase = 3;
        base + self.0 as usize * increase
    }

    #[expect(dead_code)]
    pub fn temp_increase(&self) -> u8 {
        self.0.max(3)
    }

    #[expect(dead_code)]
    pub fn payout(&self) -> usize {
        match self.0 {
            ..=1 => 2,
            2 => 3,
            3 => 5,
            4 => 8,
            _ => 12,
        }
    }
}
