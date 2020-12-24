use bevy::prelude::*;
use bevy_stl::StlPlugin;
use core::f32::consts::PI;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(StlPlugin)
        .add_resource(SpinTimer(Timer::from_seconds(1.0 / 60.0, true)))
        .add_startup_system(setup.system())
        .add_system(spin_disc.system())
        .run();
}

struct Disc {
    angle: f32,
}

struct SpinTimer(Timer);

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn(PbrBundle {
            mesh: asset_server.load("models/disc.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            ..Default::default()
        })
        .with(Disc { angle: 0.0 })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 100.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 100.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::unit_y()),
            ..Default::default()
        });
}

fn spin_disc(time: Res<Time>, mut timer: ResMut<SpinTimer>, mut query: Query<(&mut Disc, &mut Transform)>) {
    if timer.0.tick(time.delta_seconds()).just_finished() {
        for (mut disc, mut transform) in query.iter_mut() {
            disc.angle = disc.angle + 0.3 * PI/180.0;
            *transform = Transform::from_rotation(Quat::from_rotation_z(disc.angle));
        }
    }
}
