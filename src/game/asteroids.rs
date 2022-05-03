use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use rand::prelude::*;
use crate::game::GameState;
use crate::game::pause::handle_start_pause;

pub struct AsteroidsPlugin;

const PLAYER_ACCELERATION: f32 = 50.0;
const PLAYER_DECELERATION: f32 = 0.2;
const PLAYER_ROT_ACC: f32 = 2.0;
const PLAYER_ROT_DEC: f32 = 0.5;

#[derive(Component, Clone, Copy)]
struct AsteroidsItem;

#[derive(Component, Clone, Copy)]
struct PlayerCamera;

#[derive(Component, Clone, Copy)]
struct Player {
    velocity: Vec2,
    rotation: f32,
}

struct AsteroidsAtlas {
    atlas_handle: Handle<TextureAtlas>,
}

struct AsteroidsStats {
    target_number: u32,
    current_number: u32,
    spawn_timer: Timer,
}

#[derive(Component, Clone, Copy)]
struct Asteroid {
    velocity: Vec2,
    rotation: f32,
}

#[derive(Component)]
struct LaserShooter {
    cooldown: Timer,
    offset: f32,
}

impl LaserShooter {
    pub const MAX_COOLDOWN: Duration = Duration::from_millis(200);
    pub const SPEED: f32 = 200.0;
    pub const LIFETIME: Duration = Duration::from_secs(5);
}

#[derive(Component)]
struct LaserBullet {
    velocity: Vec2,
    life_time: Timer,
}

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Asteroids)
                    .with_system(asteroids_setup)
                    .with_system(spawn_player)
                    .with_system(spawn_background)
                    .with_system(spawn_ui)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Asteroids)
                    .with_system(on_exit)
                    .with_system(remove_player)
                    .with_system(remove_asteroids_atlas)
                    .with_system(remove_asteroids)
                    .before("update")
            )
            .add_system_set(
                SystemSet::on_update(GameState::Asteroids)
                    .with_system(acceleration)
                    .with_system(rotation
                                 .after(acceleration)
                    )
                    .with_system(camera_follow
                                 .after(acceleration)
                    )
                    .with_system(player_shoot_laser
                                 .after(rotation)
                    )
                    .with_system(laser_movement)
                    .with_system(laser_despawner
                                 .after(laser_movement)
                    )
                    .with_system(asteroid_number_timer)
                    .with_system(spawn_asteroid
                                 .after(asteroid_number_timer)
                    )
                    .with_system(asteroid_rotation)
                    .with_system(asteroid_movement)
                    .with_system(asteroid_distance_cleanup)
                    .with_system(handle_start_pause)
                    .label("update")
            );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

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
        })
        .insert(AsteroidsItem);
}

fn asteroids_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("images/asteroids.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(15.0, 15.0), 2, 2, Vec2::new(1.0, 1.0));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(AsteroidsAtlas { atlas_handle: texture_atlas_handle });
    commands
        .insert_resource(
            AsteroidsStats {
                target_number: 1,
                current_number: 0,
                spawn_timer: Timer::from_seconds(5.0, true),
            });
}

fn remove_asteroids_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asteroids_atlas: Res<AsteroidsAtlas>
) {
    texture_atlases.remove(asteroids_atlas.atlas_handle.clone());

    commands.remove_resource::<AsteroidsAtlas>();
    commands.remove_resource::<AsteroidsStats>();
}

fn asteroid_number_timer(mut asteroids_stats: ResMut<AsteroidsStats>, time: Res<Time>) {
    asteroids_stats.spawn_timer.tick(time.delta());

    if asteroids_stats.spawn_timer.finished() {
        // Logic to shorten time or something
        asteroids_stats.spawn_timer.set_duration(Duration::from_secs(5));

        asteroids_stats.target_number += 1;
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    asteroids_atlas: Res<AsteroidsAtlas>,
    mut asteroids_stats: ResMut<AsteroidsStats>,
    query: Query<&Transform, With<Player>>
) {

    if asteroids_stats.current_number < asteroids_stats.target_number {

        let player_translation = query.single().translation;
        let mut rng = rand::thread_rng();

        let offset_angle = rng.gen_range(0.0..2.0*PI);
        let asteroid_offset = vec2_from_circle(offset_angle, rng.gen_range(200.0..300.0));
        let translation = player_translation + Vec3::new(asteroid_offset.x, asteroid_offset.y, 0.5);

        let rotation = rng.gen_range(-0.7..0.7);
        let angle = offset_angle + PI + rng.gen_range(-0.5..0.5);
        let speed = rng.gen_range(40.0..80.0);
        let velocity = vec2_from_circle(angle, speed);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: asteroids_atlas.atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: rand::thread_rng().gen_range(0..3),
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Asteroid {
                velocity,
                rotation,
            });

        asteroids_stats.current_number += 1;
    }

}

fn asteroid_distance_cleanup(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    player_query: Query<&Transform, With<Player>>
) {

    if !asteroid_query.is_empty() {
        let player_translation = player_query.single().translation;

        asteroid_query.for_each(|(entity, transform)| {
            if player_translation.distance(transform.translation) > 400.0 {
                commands.entity(entity).despawn();
            }
        });
    }
}

fn remove_asteroids(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {

    let texture_handle = asset_server.load("images/stars.png");

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(256.0, 256.0)),
                anchor: bevy::sprite::Anchor::Center,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            texture: texture_handle,
            ..Default::default()
        });

}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font_handle = asset_server.load("fonts/Regular.ttf");

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(AsteroidsItem);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(AsteroidsItem)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Score: 0",
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 35.0,
                                    color: Color::rgb(0.9, 0.9, 0.9)
                                },
                                Default::default()
                            ),
                            ..Default::default()
                        });
                });
        });
}

fn remove_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    player_camera_query: Query<Entity, With<PlayerCamera>>,
) {
    commands.entity(player_query.single()).despawn();
    commands.entity(player_camera_query.single()).despawn();
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<AsteroidsItem>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

fn rotation(
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

fn acceleration(
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

fn camera_follow(
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

fn asteroid_rotation(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    asteroid_query.for_each_mut(|(mut transform, asteroid)| {
        let rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), asteroid.rotation * time.delta_seconds());
        transform.rotation = transform.rotation.mul_quat(rotation);
    });
}

fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    asteroid_query.for_each_mut(|(mut transform, asteroid)| {
        transform.translation.x += asteroid.velocity.x * time.delta_seconds();
        transform.translation.y += asteroid.velocity.y * time.delta_seconds();
    });
}

fn player_shoot_laser(
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

fn laser_movement(mut query: Query<(&mut Transform, &LaserBullet)>, time: Res<Time>) {
    query.for_each_mut(|(mut transform, laser_bullet)| {
        transform.translation.x += laser_bullet.velocity.x * time.delta_seconds();
        transform.translation.y += laser_bullet.velocity.y * time.delta_seconds();
    });
}

fn laser_despawner(
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
