use crate::components::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let orthographic_projection = OrthographicProjection {
        scale: 1.,
        scaling_mode: ScalingMode::Auto{min_width: 368., min_height: 208.},
        ..Default::default()
    };

    commands.spawn(Camera2dBundle {
        projection: orthographic_projection,
        ..Default::default()
    });

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        transform: Transform::from_xyz(-368./2., -208./2., 0.),
        ..Default::default()
    });
}

pub fn move_player(keyboard_input: Res<Input<KeyCode>>, 
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>) {

    for mut controller in controllers.iter_mut() {

        if keyboard_input.pressed(KeyCode::Left) {
            controller.translation = Some(Vec2::new(-1.0, 0.));
            println!("left");
        }

        if keyboard_input.pressed(KeyCode::Right) {
            controller.translation = Some(Vec2::new(1.0, 0.));
            println!("right");
        }

        if keyboard_input.pressed(KeyCode::Up) {
            controller.translation = Some(Vec2::new(0., 1.0));
            println!("up");
        }

        if keyboard_input.pressed(KeyCode::Down) {
            controller.translation = Some(Vec2::new(0., -1.0));
            println!("down");
        }

    }
}