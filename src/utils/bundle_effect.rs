// grabbed from https://github.com/BraymatterOrg/bevy_jamkit/blob/fe0d8bba5e9eba6f90e8b8796f99ebfe5442d53e/src/lib.rs#L29-L56

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.world_mut()
        .register_component_hooks::<BundleEffect>()
        .on_insert(|mut world, ctx| {
            let mut e = world.entity_mut(ctx.entity);
            let mut effect = e
                .get_mut::<BundleEffect>()
                .expect("This should be here, it's in its own hook");
            let f = std::mem::replace(&mut effect.0, Box::new(|_world: &mut EntityCommands| {}));
            let mut cmds = world.commands();
            f(&mut cmds.entity(ctx.entity));
            cmds.entity(ctx.entity).remove::<BundleEffect>();
        });
}

#[derive(Component)]
pub struct BundleEffect(pub Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>);
impl BundleEffect {
    pub fn new(f: impl 'static + FnOnce(&mut EntityCommands) + Send + Sync) -> Self {
        Self(Box::new(f))
    }
}
