mod pieces;
mod board;
mod my_colors;

use bevy::prelude::*;
use bevy_mod_picking::*;
use pieces::*;
use board::*;

fn main() {

    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 1000,
            height: 1000,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-8., 20.0, 4.0),
            )),
            ..Default::default()
        })
        .with(PickSource::default())

        // Light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
            ..Default::default()
        });
}




