use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use rand::prelude::*;
use crate::game::GameState;
use crate::game::pause::handle_start_pause;

mod player;

pub struct AsteroidsPlugin;


#[derive(Component, Clone, Copy)]
struct AsteroidsItem;

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

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Asteroids)
                    .with_system(asteroids_setup)
                    .with_system(player::spawn_player)
                    .with_system(spawn_background)
                    .with_system(spawn_ui)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Asteroids)
                    .with_system(on_exit)
                    .with_system(player::remove_player)
                    .with_system(remove_asteroids_atlas)
                    .with_system(remove_asteroids
                                 .before(remove_asteroids_atlas)
                    )
                    .before("update")
            )
            .add_system_set(
                SystemSet::on_update(GameState::Asteroids)
                    .with_system(player::acceleration)
                    .with_system(player::rotation
                                 .after(player::acceleration)
                    )
                    .with_system(player::camera_follow
                                 .after(player::acceleration)
                    )
                    .with_system(player::player_shoot_laser
                                 .after(player::rotation)
                    )
                    .with_system(player::laser_movement)
                    .with_system(player::laser_despawner
                                 .after(player::laser_movement)
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
    query: Query<&Transform, With<player::Player>>
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
    player_query: Query<&Transform, With<player::Player>>
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

fn on_exit(mut commands: Commands, query: Query<Entity, With<AsteroidsItem>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
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

fn vec2_from_circle(angle: f32, radius: f32) -> Vec2 {
    Vec2::new(-angle.sin() * radius, angle.cos() * radius)
}
