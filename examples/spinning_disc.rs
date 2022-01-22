use bevy::{prelude::*, utils::Duration};
use bevy_stl::StlPlugin;
use core::f32::consts::PI;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(StlPlugin)
        .insert_resource(SpinTimer(Timer::from_seconds(1.0 / 60.0, true)))
        .add_startup_system(setup)
        .add_system(spin_disc)
        .run();
}

#[derive(Component)]
struct Disc {
    angle: f32,
}

struct SpinTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/disc.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            ..Default::default()
        })
        .insert(Disc { angle: 0.0 });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(30.0, 0.0, 20.0),
        point_light: PointLight {
            range: 40.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -100.0, 100.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn spin_disc(
    time: Res<Time>,
    mut timer: ResMut<SpinTimer>,
    mut query: Query<(&mut Disc, &mut Transform)>,
) {
    if timer
        .0
        .tick(Duration::from_secs_f32(time.delta_seconds()))
        .just_finished()
    {
        for (mut disc, mut transform) in query.iter_mut() {
            disc.angle = disc.angle + 0.3 * PI / 180.0;
            *transform = Transform::from_rotation(Quat::from_rotation_z(disc.angle));
        }
    }
}
