use std::time::Duration;
use bevy::prelude::*;
use crate::game::GameState;
use crate::game::pause::handle_start_pause;

pub struct AsteroidsPlugin;

const PLAYER_ACCELERATION: f32 = 1.0;
const PLAYER_DECELERATION: f32 = 0.2;
const PLAYER_ROT_ACC: f32 = 0.05;
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
    spawn_delay: Duration,
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
                    .with_system(remove_asteroids_atlas)
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
                    .with_system(spawn_asteroid)
                    .with_system(handle_start_pause)
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

    commands.insert_resource(AsteroidsAtlas { atlas_handle: texture_atlas_handle.clone() });
    commands
        .insert_resource(
            AsteroidsStats {
                target_number: 1,
                current_number: 0,
                spawn_delay: Duration::from_secs(2),
            });
}

fn remove_asteroids_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asteroids_atlas: Res<AsteroidsAtlas>
) {
    texture_atlases.remove(asteroids_atlas.atlas_handle.clone());
    commands.remove_resource::<AsteroidsAtlas>();
}

fn spawn_asteroid(
    mut commands: Commands,
    asteroids_atlas: Res<AsteroidsAtlas>,
    mut asteroids_stats: ResMut<AsteroidsStats>
) {

    if asteroids_stats.current_number < asteroids_stats.target_number {

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: asteroids_atlas.atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(50.0, 50.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            });

        asteroids_stats.current_number += 1;
    }

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
                                    color: Color::rgb(0.9, 0.9, 0.9).into()
                                },
                                Default::default()
                            ),
                            ..Default::default()
                        });
                });
        });
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
        if player.rotation > -0.05 {
            player.rotation -= PLAYER_ROT_ACC * time.delta().as_secs_f32();
        }
        rotated = true;
    }

    if keys.pressed(KeyCode::A) {
        if player.rotation < 0.05 {
            player.rotation += PLAYER_ROT_ACC * time.delta().as_secs_f32();
        }
        rotated = true;
    }

    if !rotated {
        let reduction = player.rotation * PLAYER_ROT_DEC;
        player.rotation -= reduction * time.delta().as_secs_f32();
    }

    let last_rot = transform.rotation.to_euler(EulerRot::ZYX);
    transform.rotation = Quat::from_euler(EulerRot::ZYX, last_rot.0 + player.rotation, 0.0, 0.0);
}

fn acceleration(
    mut query: Query<(&mut Transform, &mut Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut transform, mut player) = query.single_mut();
    let mut accelerated = false;

    let rotation = transform.rotation.to_euler(EulerRot::ZYX);
    let direction_vec = Vec2::new(-rotation.0.sin(), rotation.0.cos());

    if keys.pressed(KeyCode::W) {
        let acc = direction_vec * (PLAYER_ACCELERATION * time.delta().as_secs_f32());
        player.velocity += acc;
        accelerated = true;
    }

    if keys.pressed(KeyCode::S) {
        let acc = direction_vec * (PLAYER_ACCELERATION * time.delta().as_secs_f32());
        player.velocity -= acc;
        accelerated = true;
    }

    if !accelerated {
        let reduction = player.velocity * PLAYER_DECELERATION;
        player.velocity -= reduction * time.delta().as_secs_f32();
    }

    transform.translation.x += player.velocity.x;
    transform.translation.y += player.velocity.y;
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
