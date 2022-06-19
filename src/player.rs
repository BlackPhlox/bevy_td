use std::f32::consts::{FRAC_PI_2, PI};

use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::{
    math::{Vec2Swizzles, Vec3Swizzles},
    prelude::{shape::Quad, *},
};
use bevy_prototype_lyon::{prelude::*, shapes::Line};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct MousePos(Vec2);

#[derive(Component, Debug)]
pub struct Bullet {
    pub pos_vel: Line,
    pub prev: Vec2,
    pub lifetime: u32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            pos_vel: Line(Vec2::ZERO, Vec2::ZERO),
            prev: Vec2::ZERO,
            lifetime: Default::default(),
        }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                .with_system(cursor_grab_system)
                .with_system(shooting_system)
                .with_system(bullet_system)
                .with_system(bullet_draw)
                .with_system(bullet_delete_system),
        );
    }
}

fn cursor_grab_system(mut windows: ResMut<Windows>, actions: Res<Actions>) {
    let window = windows.get_primary_mut().unwrap();
    if actions.grabbed_mouse {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    } else {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
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

            //let t = player_transform.looking_at(mouse_pos.extend(0.), Vec3::Y);

            //let q = Quat::from_axis_angle(Vec3::Y, player_pos.xy().heading());
            //let mut v = (t.translation).xy();

            let c = Bullet {
                pos_vel: Line(player_pos.xy(), v * 0.1),
                prev: player_pos.xy(),
                lifetime: 0,
            };

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &Line(player_pos.xy(), player_pos.xy()),
                    DrawMode::Stroke(StrokeMode {
                        options: so,
                        color: Color::BLACK,
                    }),
                    Transform::default(),
                ))
                .insert(c);
        }
    }
}

fn bullet_system(mut bullet_query: Query<&mut Bullet>) {
    for mut bullet in bullet_query.iter_mut() {
        bullet.lifetime += 1;
        let v = bullet.pos_vel.1;
        //bullet.pos_vel.1 = Vec2::ONE;
        bullet.prev = bullet.pos_vel.0;
        bullet.pos_vel.0 += v;
    }
}

fn bullet_delete_system(mut commands: Commands, bullet_query: Query<(Entity, &Bullet)>) {
    for (e, bullet) in bullet_query.iter() {
        if bullet.lifetime > 100 {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn bullet_draw(mut pb_query: Query<(&mut Path, &mut Bullet)>) {
    for (mut path, bullet) in pb_query.iter_mut() {
        *path = ShapePath::build_as(&Line(bullet.prev, bullet.pos_vel.0));
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
                custom_size: Some(Vec2::new(16., 16.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);

    let lines = shapes::Line(Vec2::new(-100.0, 0.0), Vec2::new(200.0, 200.0));

    commands.spawn().insert(MousePos(Vec2::new(0.0, 0.0)));

    let mut so = StrokeOptions::default();
    so.line_width = 3.0;

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

#[allow(unused_mut)]
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut world_query: Query<&mut Transform, Without<Player>>,
    mut line_query: Query<&mut Path, With<LineShape>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let mut delta_mouse_pos = Vec2::ZERO;

    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
        delta_mouse_pos = _position;
    }

    if actions.player_movement.is_none() {
        update_line(
            delta_mouse_pos,
            player_query.single().translation.xy(),
            window,
            line_query,
        );
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

    for mut transform in world_query.iter_mut() {
        transform.translation += movement;
    }

    update_line(
        delta_mouse_pos,
        player_query.single().translation.xy(),
        window,
        line_query,
    );
}

fn update_line(
    delta_mouse_pos: Vec2,
    player_pos: Vec2,
    window: &Window,
    mut line_query: Query<&mut Path, With<LineShape>>,
) {
    let mut p = delta_mouse_pos - Vec2::new(window.width() / 2., window.height() / 2.);
    p = p.clamp_length(13., 13.);
    p += player_pos;

    for mut path in line_query.iter_mut() {
        let polygon = shapes::Line(Vec2::new(player_pos.x, player_pos.y), Vec2::new(p.x, p.y));
        *path = ShapePath::build_as(&polygon);
    }
}
