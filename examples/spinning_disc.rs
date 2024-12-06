use bevy::{prelude::*, utils::Duration};
use bevy_stl::StlPlugin;
use core::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StlPlugin)
        .insert_resource(SpinTimer(Timer::from_seconds(
            1.0 / 60.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, spin_disc)
        .run();
}

#[derive(Component)]
struct Disc {
    angle: f32,
}

#[derive(Resource)]
struct SpinTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(asset_server.load("models/disc.stl")),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.4, 0.3))),
        Transform::from_rotation(Quat::from_rotation_z(0.0)),
        Disc { angle: 0.0 },
    ));
    commands.spawn((
        Transform::from_xyz(30.0, 0.0, 20.0),
        PointLight {
            range: 40.0,
            ..Default::default()
        },
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -100.0, 100.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Camera3d::default(),
        Msaa::Sample4,
    ));
}

fn spin_disc(
    time: Res<Time>,
    mut timer: ResMut<SpinTimer>,
    mut query: Query<(&mut Disc, &mut Transform)>,
) {
    if timer
        .0
        .tick(Duration::from_secs_f32(time.delta_secs()))
        .just_finished()
    {
        for (mut disc, mut transform) in query.iter_mut() {
            disc.angle = disc.angle + 0.3 * PI / 180.0;
            *transform = Transform::from_rotation(Quat::from_rotation_z(disc.angle));
        }
    }
}
