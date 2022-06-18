// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::f64::consts::PI;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::*;
use bevy_game::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(change_draw_mode_system)
        .add_system(change_number_of_sides)
        .add_system(rotate_shape_system)
        .run();
}

#[derive(Component)]
struct ExampleShape;

#[derive(Component)]
struct LineShape;

fn rotate_shape_system(mut query: Query<&mut Transform, With<ExampleShape>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}

fn change_draw_mode_system(mut query: Query<&mut DrawMode>, time: Res<Time>) {
    let hue = (time.seconds_since_startup() * 50.0) % 360.0;
    let outline_width = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;

    for mut draw_mode in query.iter_mut() {
        if let DrawMode::Outlined {
            ref mut fill_mode,
            ref mut outline_mode,
        } = *draw_mode
        {
            fill_mode.color = Color::hsl(hue as f32, 1.0, 0.5);
            outline_mode.options.line_width = outline_width as f32;
        }
    }
}

fn change_number_of_sides(mut query: Query<&mut Path, With<ExampleShape>>, time: Res<Time>) {
    let sides = ((time.seconds_since_startup() - PI * 2.5).sin() * 2.5 + 5.5).round() as usize;

    for mut path in query.iter_mut() {
        let polygon = shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(200.0),
            ..shapes::RegularPolygon::default()
        };

        *path = ShapePath::build_as(&polygon);
    }
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::default(),
        ))
        .insert(ExampleShape);

    let lines = shapes::Line {
        0: Vec2::new(-100.0, 0.0),
        1: Vec2::new(200.0, 200.0),
    };

    commands.spawn_bundle(GeometryBuilder::build_as(&lines, DrawMode::Stroke(StrokeMode::color(Color::BLACK)), Transform::default())).insert(LineShape);
}