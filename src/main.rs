use bevy::prelude::*;

const UP_SPEED: f32 = 500.0;
const GRAVITY: f32 = -2000.0;
const ANGLE_AMPLITUDE: f32 = 0.8;
const PIPE_WINDOW_SIZE: f32 = 200.0;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    velocity: Velocity,
    sprite: SpriteBundle,
    marker: Player,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, images: Res<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle {
        sprite: SpriteBundle {
            texture: asset_server.load("sprites/bird.png"),
            ..default()
        },
        velocity: Velocity(Vec3::ZERO),
        marker: Player,
    });

    let pipe_start = asset_server.load("sprites/pipe.png");
    let pipe_start_height = images.get(&pipe_start).unwrap().size().y;

    let pipe_segment = asset_server.load("sprites/pipe_piece.png");
    let pipe_segment_height = images.get(&pipe_segment).unwrap().size().y;

    let lower_pipe_bundle = SpriteBundle {
        texture: pipe_start,
        transform: Transform {
            translation: Vec3::Y * -(pipe_start_height + PIPE_WINDOW_SIZE)/2.0,
            ..default()
        },
        ..default()
    };

    let mut higher_pipe_bundle = lower_pipe_bundle.clone();
    higher_pipe_bundle.sprite.flip_y = true;
    higher_pipe_bundle.transform.translation *= -1.0;

    commands.spawn(lower_pipe_bundle);
    commands.spawn(higher_pipe_bundle);
}

fn jump(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        let mut velocity = query.single_mut();
        velocity.0 = Vec3::Y * UP_SPEED;
    }
}

fn rotate(mut query: Query<(&mut Transform, &Velocity), With<Player>>) {
    for (mut transform, velocity) in &mut query {
        use std::f32::consts::FRAC_PI_2;
        let angle = ((velocity.0.y / UP_SPEED) * ANGLE_AMPLITUDE).clamp(-FRAC_PI_2, FRAC_PI_2);
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    let dt = time.delta_seconds();
    for mut velocity in &mut query {
        velocity.0 -= Vec3::NEG_Y * GRAVITY * dt;
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0 * dt;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_systems((
            jump,
            apply_gravity.after(jump),
            apply_velocity.after(apply_gravity),
            rotate.after(apply_gravity),
        ))
        .run();
}
