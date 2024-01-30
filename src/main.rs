use bevy::{prelude::*, render::camera::ScalingMode, window::*};
use bevy_prototype_lyon::prelude::*;

const BALL_RADIUS: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (setup, gravity.after(setup)))
        .add_systems(Update, update_position)
        .run();
} 

#[derive(Component)]
struct Ball2D;

#[derive(Component, Default)]
struct Mass(f32);

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component, Default)]
struct Force(Vec3);

fn gravity(mut query: Query<(&Mass, &mut Force), With<Ball2D>>) {
    for (mass, mut force) in query.iter_mut() {
        force.0.y -= 9.8f32 * mass.0;
    }
}

// fn test_fall(mut query: Query<&mut Transform, With<Ball2D>>, time: Res<Time>) {
//     let delta = time.elapsed_seconds();
//     for mut transform in query.iter_mut() {
//         transform.translation.y -= delta;
//         // println!("{delta}");
//     }
// }

fn update_position(mut query: Query<(&mut Transform, &Mass, &Force, &mut Velocity), With<Ball2D>>, time: Res<Time>) {
    let t = time.delta_seconds();
    if t == 0.0 {
        return;
    }
    
    for (mut transform, mass, force, mut v) in query.iter_mut() {
        // println!("Force on ball {}", force.0);

        let a = force.0 / mass.0;
        // println!("Acceleration of ball is {a}");
        
        let x1 = transform.translation;
        // println!("Current position of ball is {x1}");
        // println!("Current velocity of ball is {}", v.0);
        
        let x2 = x1 + v.0*t + (0.5)*a*(t*t);
        v.0 = (x2 - x1) * (1.0/t);
        // println!("New velocity of ball is {}", v.0);

        // println!("Ball is now at position {x2}");
        transform.translation = x2;

        // println!("");
    }
}

fn keep_in_window(mut query: Query<(&Transform, &mut Velocity), With<Ball2D>>) {
    
}

fn setup(mut commands: Commands) {
    let projection = OrthographicProjection {
        far: 100.,
        near: -100.,
        scaling_mode: ScalingMode::WindowSize(6.0),
        ..Default::default()};

    commands.spawn(Camera2dBundle{
        projection,
        ..Default::default()
    });
    commands.spawn((
            Force(Vec3 {x:0.0, y: -98.0, z: 0.0}),
            Velocity::default(),
            Mass(10.0),
            ShapeBundle{
            path: GeometryBuilder::build_as(&shapes::Circle{radius: BALL_RADIUS, center: Vec2{x: 0.0, y: 100.0}}),
            ..default()
            },
            // Stroke::color(Color::BLACK),
            Fill::color(Color::WHITE),
            Ball2D,
    ));
}