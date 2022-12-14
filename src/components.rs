use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle]
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
    pub rotation_constraints: LockedAxes,
    // pub velocity: Velocity,
    // pub gravity_scale: GravityScale,
    // pub friction: Friction,
    // pub density: ColliderMassProperties,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(7., 7.),
                controller: KinematicCharacterController::default(),
                rigid_body: RigidBody::KinematicPositionBased,
                rotation_constraints,
                // friction: Friction {
                //     coefficient: 0.0,
                //     combine_rule: CoefficientCombineRule::Min,
                // },
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    #[bundle]
    pub collider_bundle: TerrainColliderBundle,
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct TerrainColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub rotation_constraints: LockedAxes,
    // pub velocity: Velocity,
    // pub gravity_scale: GravityScale,
    // pub friction: Friction,
    // pub density: ColliderMassProperties,
}

impl Default for TerrainColliderBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(4., 4.),
            rigid_body: RigidBody::Fixed,
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
        }
    }

}
