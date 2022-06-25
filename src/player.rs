use std::f32::consts::{FRAC_PI_2, PI, TAU};

use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::{
    math::{Vec2Swizzles, Vec3Swizzles},
    prelude::{shape::Quad, *},
};
use bevy_prototype_lyon::{
    prelude::{
        tess::geom::{euclid::Point2D, Point},
        DrawMode, FillOptions, GeometryBuilder, Path, RectangleOrigin, ShapePath, StrokeMode,
        StrokeOptions,
    },
    shapes::{self, Circle, Line},
};

use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct MousePos(Vec2);

#[derive(Component, Debug)]
pub struct Bullet {
    pub lifetime: u32,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_physics)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                //.with_system(cursor_grab_system)
                .with_system(shooting_system)
                //.with_system(bullet_draw)
                .with_system(bullet_delete_system)
                .with_system(rotate_system)
                .with_system(bullet_current_system)
                .with_system(zombie_spawner)
                .with_system(zombie_nav)
                .with_system(zombie_despawn),
        );
    }
}

fn setup_physics(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut r = rand::thread_rng();

    for i in -5..5 {
        let a = r.gen_range(0.0..TAU);

        commands
            .spawn()
            .insert(Collider::cuboid(100.0, 5.0))
            .insert_bundle(GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::new(100.0 * 2., 5.0),
                    origin: RectangleOrigin::Center,
                },
                DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode {
                    options: FillOptions::default(),
                    color: Color::GRAY,
                }),
                Transform {
                    translation: Vec3::new(100.0 * i as f32, -100.0, 0.0),
                    rotation: Quat::from_rotation_z(a),
                    ..Default::default()
                },
            ));
    }

    let wh_size = 64.;

    for y in -10..10 {
        for x in -10..10 {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.texture_ground.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * wh_size,
                        y as f32 * wh_size,
                        0.,
                    )),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(wh_size, wh_size)),
                        color: Color::rgba(0.5, 0.5, 0.5, 0.5),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile);
        }
    }
}

#[derive(Component)]
struct Tile;

#[allow(dead_code)]
fn cursor_grab_system(mut windows: ResMut<Windows>, actions: Res<Actions>) {
    let window = windows.get_primary_mut().unwrap();
    if actions.grabbed_mouse.0 {
        if actions.grabbed_mouse.1 {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        } else {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }
    }
}

#[derive(Component)]
struct Zombie {
    health: u32,
}

fn zombie_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut last_time: Local<f64>,
    windows: Res<Windows>,
) {
    let lst = time.seconds_since_startup() - *last_time;
    let window = windows.get_primary().unwrap();
    let mut r = rand::thread_rng();
    let a = r.gen_range(0.0..window.width());
    if lst > 1.0 {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.))
            .insert(Collider::ball(8.0))
            .insert(Restitution::coefficient(0.99))
            .insert(Sleeping::disabled())
            .insert(ColliderMassProperties::Density(5.0))
            .insert_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: 8.0,
                    center: Vec2::new(0., 0.),
                },
                DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode {
                    options: FillOptions::default(),
                    color: Color::RED,
                }),
                Transform {
                    translation: Vec3::new(a - window.width() / 2., -100., 0.0),
                    ..Default::default()
                },
            ))
            .insert(Zombie { health: 100 });
        *last_time = time.seconds_since_startup();
    }
}

fn zombie_nav(
    player_query: Query<&Transform, (With<Player>, Without<Zombie>)>,
    mut zombie_query: Query<&mut Transform, (With<Zombie>, Without<Player>)>,
) {
    let player_pos = player_query.single();
    for mut zombie in zombie_query.iter_mut() {
        let mut dir = player_pos.translation - zombie.translation;
        dir = dir.normalize();
        zombie.translation += dir * 0.5;
    }
}

fn zombie_despawn(
    mut commands: Commands,
    mut zombie_query: Query<(Entity, &Zombie)>,
) {
    for (e, zombie) in zombie_query.iter_mut() {
        if zombie.health <= 0 {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub trait Heading {
    fn heading(&self) -> f32;
}

impl Heading for Vec2 {
    fn heading(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

fn shooting_system(
    mut commands: Commands,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    windows: Res<Windows>,
) {
    if actions.trigger_pressed {
        let window = windows.get_primary().unwrap();
        let mut mouse_pos = Vec2::ZERO;

        if let Some(_position) = window.cursor_position() {
            // cursor is inside the window, position given
            mouse_pos = _position;
        }

        let mut so = StrokeOptions::default();
        so.line_width = 1.0;
        let p = player_query.get_single();
        if let Ok(player_transform) = p {
            let player_pos = player_transform.translation;

            let relative_mouse_world_pos =
                mouse_pos - Vec2::new(window.width() / 2., window.height() / 2.);
            let player_world_pos = relative_mouse_world_pos + player_pos.xy();
            let heading = player_world_pos.y.atan2(player_world_pos.x);

            let q = Quat::from_axis_angle(Vec3::Y, heading);
            let mut v = (q * Vec2::new(player_world_pos.x, player_world_pos.y).extend(0.)).xy();

            if relative_mouse_world_pos.x - player_pos.x < 0. {
                v = Vec2::new(v.x * -1.0, v.y);
            }

            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.))
                .insert(Collider::ball(0.5))
                //.insert(ActiveCollisionTypes::DYNAMIC_KINEMATIC)
                .insert(Restitution::coefficient(0.99))
                .insert(Sleeping::disabled())
                .insert(ColliderMassProperties::Density(1.0))
                //.insert(Dominance::group(10))
                .insert(Velocity {
                    linvel: v * 5.,
                    ..default()
                })
                .insert(Ccd::enabled())
                .insert(Bullet {
                    lifetime: 0,
                })
                .insert_bundle(TransformBundle::from_transform(Transform::from_translation(player_pos)));
        }
    }
}

fn bullet_current_system(mut bullet_query: Query<(&Velocity, &mut Bullet), With<Bullet>>) {
    for (vel, mut bullet) in bullet_query.iter_mut() {
        bullet.lifetime += 1;
        if vel.linvel.length_squared() < 80. {
            bullet.lifetime = 50;
        }
    }
}

fn bullet_delete_system(mut commands: Commands, bullet_query: Query<(Entity, &Bullet)>) {
    for (e, bullet) in bullet_query.iter() {
        if bullet.lifetime > 50 {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct LineShape;

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64., 64.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);

    commands.spawn().insert(MousePos(Vec2::new(0.0, 0.0)));

    let mut so = StrokeOptions::default();
    so.line_width = 3.0;

    let mut r = rand::thread_rng();

    for i in -10..10 {
        let w = r.gen_range(0.0..100.);
        let h = r.gen_range(0.0..100.);
        let w1 = r.gen_range(0.0..100.);
        let h1 = r.gen_range(0.0..100.);
        let lines = shapes::Line(Vec2::new(w * i as f32, h * i as f32), Vec2::new(w1 * i as f32, h1 * i as f32));
        commands
        .spawn_bundle(GeometryBuilder::build_as(
            &lines,
            DrawMode::Stroke(StrokeMode {
                options: so,
                color: Color::BLACK,
            }),
            Transform::default(),
        ))
        .insert(LineShape);
    }
}

fn rotate_system(mut query: Query<(&mut Transform, With<Player>)>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let mut delta_mouse_pos = Vec2::ZERO;

    if let Some(_position) = window.cursor_position() {
        delta_mouse_pos = _position;
    }

    for (mut transform, is_rotation_entity) in query.iter_mut() {
        let player_pos = transform.translation;

        let relative_mouse_world_pos =
            delta_mouse_pos - Vec2::new(window.width() / 2., window.height() / 2.);
        let player_world_pos = relative_mouse_world_pos + player_pos.xy();
        let heading = player_world_pos.y.atan2(player_world_pos.x);

        if is_rotation_entity {
            transform.rotation = Quat::from_rotation_z(heading - FRAC_PI_2);
        }
    }
}

#[allow(unused_mut)]
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
