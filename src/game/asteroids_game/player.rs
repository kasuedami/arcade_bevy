use bevy::prelude::*;
use std::time::Duration;

const PLAYER_ACCELERATION: f32 = 50.0;
const PLAYER_DECELERATION: f32 = 0.2;
const PLAYER_ROT_ACC: f32 = 2.0;
const PLAYER_ROT_DEC: f32 = 0.5;

#[derive(Component, Clone, Copy)]
pub struct Player {
    velocity: Vec2,
    rotation: f32,
}

#[derive(Component, Clone, Copy)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct LaserShooter {
    cooldown: Timer,
    offset: f32,
}

impl LaserShooter {
    pub const MAX_COOLDOWN: Duration = Duration::from_millis(200);
    pub const SPEED: f32 = 200.0;
    pub const LIFETIME: Duration = Duration::from_secs(5);
}

#[derive(Component)]
pub struct LaserBullet {
    velocity: Vec2,
    life_time: Timer,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut ortho_camera = OrthographicCameraBundle::new_2d();
    ortho_camera.orthographic_projection.scale = 0.4;

    commands
        .spawn_bundle(ortho_camera)
            .insert(PlayerCamera);

    let ship_handle = asset_server.load("images/ship.png");

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(48.0, 48.0)),
                anchor: bevy::sprite::Anchor::Center,
                ..Default::default()
            },
            texture: ship_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            velocity: Vec2::ZERO,
            rotation: 0.0,
        })
        .insert(LaserShooter {
            cooldown: Timer::new(LaserShooter::MAX_COOLDOWN, false),
            offset: 30.0,
        });
}

pub fn remove_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    player_camera_query: Query<Entity, With<PlayerCamera>>,
) {
    commands.entity(player_query.single()).despawn();
    commands.entity(player_camera_query.single()).despawn();
}

pub fn rotation(
    mut query: Query<(&mut Transform, &mut Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {

    let (mut transform, mut player) = query.single_mut();
    let mut rotated = false;

    if keys.pressed(KeyCode::D) {
        if player.rotation > -2.5 {
            player.rotation -= PLAYER_ROT_ACC * time.delta_seconds();
        }
        rotated = true;
    }

    if keys.pressed(KeyCode::A) {
        if player.rotation < 2.5 {
            player.rotation += PLAYER_ROT_ACC * time.delta_seconds();
        }
        rotated = true;
    }

    if !rotated {
        let reduction = player.rotation * PLAYER_ROT_DEC * time.delta_seconds();
        player.rotation -= reduction;
    }

    let rot = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), player.rotation * time.delta_seconds());
    transform.rotation = transform.rotation.mul_quat(rot);
}

pub fn acceleration(
    mut query: Query<(&mut Transform, &mut Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut transform, mut player) = query.single_mut();
    let mut accelerated = false;

    let rotation = transform.rotation.to_euler(EulerRot::ZYX);
    let direction_vec = vec2_from_circle(rotation.0, 1.0);

    if keys.pressed(KeyCode::W) {
        let acc = direction_vec * PLAYER_ACCELERATION * time.delta_seconds();
        player.velocity += acc;
        accelerated = true;
    }

    if keys.pressed(KeyCode::S) {
        let acc = direction_vec * PLAYER_ACCELERATION * time.delta_seconds();
        player.velocity -= acc;
        accelerated = true;
    }

    if !accelerated {
        let reduction = player.velocity * PLAYER_DECELERATION * time.delta_seconds();
        player.velocity -= reduction;
    }

    transform.translation.x += player.velocity.x * time.delta_seconds();
    transform.translation.y += player.velocity.y * time.delta_seconds();
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let player_translation = player_transform.translation.truncate();
    let camera_translation = camera_transform.translation.truncate();

    let diff_translation = player_translation - camera_translation;
    let diff_length = diff_translation.length();

    let correction_strength = (diff_length * 0.01) * (diff_length * 0.01);
    let correction = diff_translation * (correction_strength * time.delta().as_secs_f32());

    camera_transform.translation.x += correction.x;
    camera_transform.translation.y += correction.y;
}

pub fn player_shoot_laser(
    mut commands: Commands,
    mut query: Query<(&Transform, &Player, &mut LaserShooter)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {

    let (player_transform, player, mut laser_shooter) = query.single_mut();
    laser_shooter.cooldown.tick(time.delta());

    if keys.pressed(KeyCode::Space) && laser_shooter.cooldown.finished() {

        laser_shooter.cooldown.reset();

        let texture_handle = asset_server.load("images/laser.png");

        let rotation = player_transform.rotation.to_euler(EulerRot::ZYX);
        let direction = vec2_from_circle(rotation.0, 1.0);
        let velocity = Vec2::new(player.velocity.x + direction.x * LaserShooter::SPEED, player.velocity.y + direction.y * LaserShooter::SPEED);

        let translation = Vec3::new(
            player_transform.translation.x + direction.x * laser_shooter.offset,
            player_transform.translation.y + direction.y * laser_shooter.offset,
            player_transform.translation.z
        );

        let transform = Transform {
            translation,
            rotation: player_transform.rotation,
            scale: player_transform.scale
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                transform,
                texture: texture_handle,
                ..Default::default()
            })
            .insert(LaserBullet {
                velocity,
                life_time: Timer::new(LaserShooter::LIFETIME, false),
            });
    }

}

pub fn laser_movement(mut query: Query<(&mut Transform, &LaserBullet)>, time: Res<Time>) {
    query.for_each_mut(|(mut transform, laser_bullet)| {
        transform.translation.x += laser_bullet.velocity.x * time.delta_seconds();
        transform.translation.y += laser_bullet.velocity.y * time.delta_seconds();
    });
}

pub fn laser_despawner(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LaserBullet)>,
    time: Res<Time>
) {

    query.for_each_mut(|(entity, mut laser_bullet)| {
        laser_bullet.life_time.tick(time.delta());

        if laser_bullet.life_time.finished() {
            commands.entity(entity).despawn();
        }
    });

}

fn vec2_from_circle(angle: f32, radius: f32) -> Vec2 {
    Vec2::new(-angle.sin() * radius, angle.cos() * radius)
}
