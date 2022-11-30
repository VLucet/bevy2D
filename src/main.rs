#![allow(non_snake_case)]

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(DefaultPlugins)
        
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })

        .add_startup_system(systems::setup)
        
        .add_system(systems::move_player)
        // .add_system(systems::update_level_selection) 
        
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")

        .run();
}