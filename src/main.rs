// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use getting_over_him::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: "Getting Over Him".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(25.0))
        .add_plugin(RapierDebugRenderPlugin {
            style: DebugRenderStyle {
                border_subdivisions: 0,
                collider_dynamic_color: [0., 0., 0., 1.0],
                collider_kinematic_color: [20.0, 1.0, 0.3, 0.0],
                collider_fixed_color: [30.0, 1.0, 0.4, 0.0],
                collider_parentless_color: [30.0, 1.0, 0.4, 0.0],
                impulse_joint_anchor_color: [240.0, 0.5, 0.4, 0.0],
                impulse_joint_separation_color: [0.0, 0.5, 0.4, 0.0],
                multibody_joint_anchor_color: [300.0, 1.0, 0.4, 0.0],
                multibody_joint_separation_color: [0.0, 1.0, 0.4, 0.0],
                sleep_color_multiplier: [1.0, 1.0, 0.2, 0.0],
                rigid_body_axes_length: 0.0,
                contact_depth_color: [120.0, 1.0, 0.4, 0.0],
                contact_normal_color: [0.0, 1.0, 1.0, 0.0],
                contact_normal_length: 0.3,
                collider_aabb_color: [124.0, 1.0, 0.4, 0.0],
                ..Default::default()
            },
            ..Default::default()
        })
        .run();
}
