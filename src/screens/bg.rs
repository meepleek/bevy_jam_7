use bevy::mesh::{AnnulusMeshBuilder, CircleMeshBuilder};

use crate::{prelude::*, screens::Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Splash), spawn_bg)
        .add_systems(Update, rotate_planets)
        .add_systems(OnEnter(Playing), tween_bg_overlay::<85>)
        .add_systems(OnExit(Playing), tween_bg_overlay::<25>);
}

#[derive(Component)]
pub struct Rotate(Rot2);

#[derive(Component)]
pub struct BgOverlay;

#[derive(Component)]
pub struct PlanetPivot;

fn spawn_bg(
    mut cmd: Commands,
    sprites: Res<Sprites>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rng();
    let object_count = 12;
    let circle_size = 100.;
    let mut radius_offset = 0.;
    let mut consecutive_ring_count = 0;
    let mut sun_children = Vec::with_capacity(object_count);
    let material_yellow = MeshMaterial2d(materials.add(COL_YELLOW));
    let material_orange = MeshMaterial2d(materials.add(COL_ORANGE));

    for i in 0..object_count {
        let radius = circle_size * 1.25 + i as f32 * rng.random_range(24.0..28.0) + radius_offset;
        let thickness = rng.random_range(5.0..12.0);
        let e: Entity;
        if (consecutive_ring_count < 3 && rng.random_ratio(2, 3)) || consecutive_ring_count == 0 {
            let ring = AnnulusMeshBuilder::new(radius, radius + thickness, 80).build();
            let mesh = meshes.add(ring);
            e = cmd
                .spawn((
                    Name::new("ring"),
                    Mesh2d(mesh),
                    material_yellow.clone(),
                    Transform::from_xyz(0., 0., 0.1),
                ))
                .id();
            consecutive_ring_count += 1;
        } else {
            let planet_radius = rng.random_range(10.0..20.0);
            let mesh = meshes.add(CircleMeshBuilder::new(planet_radius, 20).build());
            let rotation = Quat::from_rotation_z(rng.random_range(0f32..360.));
            let rotation_index_mult = (object_count - i) as f32 / 4.;
            let rotation_speed = rng.random_range(15f32..30.) * rotation_index_mult;
            e = cmd
                .spawn((
                    Name::new("planet_rot"),
                    PlanetPivot,
                    Transform::from_rotation(rotation),
                    Visibility::default(),
                    Rotate(Rot2::degrees(rotation_speed)),
                    children![(
                        Name::new("planet"),
                        Mesh2d(mesh),
                        material_orange.clone(),
                        Transform::from_xyz(0., radius + planet_radius / 2., 0.1)
                    )],
                ))
                .id();
            radius_offset += planet_radius;
            consecutive_ring_count = 0;
        }

        sun_children.push(e);
    }

    let sun_e = cmd
        .spawn((
            Name::new("sun"),
            Mesh2d(meshes.add(CircleMeshBuilder::new(circle_size, 50).build())),
            material_yellow.clone(),
            Transform::from_xyz(0., 90., 0.1),
        ))
        .add_children(&sun_children)
        .id();

    cmd.spawn((
        Name::new("background"),
        Transform::from_translation(Vec3::Z * -10.),
        Visibility::default(),
        children![
            (
                Name::new("bg_mountains"),
                Sprite::from_image(sprites.bg_mountains.clone()),
                // hack: using an upscaled version &scaling it down here to avoid aliasing artifacts
                Transform::from_xyz(-280., -260., 1.).with_scale(Vec2::splat(0.25).extend(1.)),
            ),
            (
                Name::new("bg_overlay"),
                BgOverlay,
                Sprite::from_color(COL_DARK.with_alpha(0.25), Vec2::splat(8000.)),
                Transform::from_translation(Vec3::Z * 10.)
            )
        ],
    ))
    .add_child(sun_e);
}

fn rotate_planets(
    mut rotate_q: Query<(&Rotate, &mut Transform), With<PlanetPivot>>,
    overlay: Single<&Sprite, With<BgOverlay>>,
    time: Res<Time>,
) {
    let speed_mult = 1. - overlay.color.alpha();
    for (rot, mut t) in &mut rotate_q {
        t.rotate_z(rot.0.as_radians() * time.delta_secs() * speed_mult);
    }
}

fn tween_bg_overlay<const OPACITY: u8>(
    mut cmd: Commands,
    overlay_q: Query<Entity, With<BgOverlay>>,
) {
    for e in overlay_q {
        or_continue!(cmd.get_entity(e)).try_insert(tween::get_relative_sprite_color_anim(
            COL_DARK.with_alpha(OPACITY as f32 / 100.),
            3_000,
            None,
        ));
    }
}
