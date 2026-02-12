use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct Hiearchy<'w, 's, TComponent: Component> {
    card_q: Query<'w, 's, &'static TComponent>,
    parent_q: Query<'w, 's, &'static ChildOf>,
}
impl<'w, 's, TComponent: Component> Hiearchy<'w, 's, TComponent> {
    pub fn get_self_or_ancestor(&self, self_or_child_e: Entity) -> Option<(Entity, &TComponent)> {
        if let Ok(card) = self.card_q.get(self_or_child_e) {
            return Some((self_or_child_e, card));
        }

        for e in self.parent_q.iter_ancestors(self_or_child_e) {
            if let Ok(card) = self.card_q.get(e) {
                return Some((e, card));
            }
        }

        None
    }
}
