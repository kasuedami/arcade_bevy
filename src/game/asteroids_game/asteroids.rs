use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;
use std::f32::consts::PI;
use super::player::Player;

#[derive(Component, Clone, Copy)]
pub struct Asteroid {
    velocity: Vec2,
    rotation: f32,
}

pub struct AsteroidsAtlas {
    atlas_handle: Handle<TextureAtlas>,
}

pub struct AsteroidsStats {
    target_number: u32,
    current_number: u32,
    spawn_timer: Timer,
}

pub fn asteroids_setup(
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

pub fn remove_asteroids_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asteroids_atlas: Res<AsteroidsAtlas>
) {
    texture_atlases.remove(asteroids_atlas.atlas_handle.clone());

    commands.remove_resource::<AsteroidsAtlas>();
    commands.remove_resource::<AsteroidsStats>();
}

pub fn asteroid_number_timer(mut asteroids_stats: ResMut<AsteroidsStats>, time: Res<Time>) {
    asteroids_stats.spawn_timer.tick(time.delta());

    if asteroids_stats.spawn_timer.finished() {
        // Logic to shorten time or something
        asteroids_stats.spawn_timer.set_duration(Duration::from_secs(5));

        asteroids_stats.target_number += 1;
    }
}

pub fn spawn_asteroid(
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

pub fn asteroid_distance_cleanup(
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

pub fn remove_asteroids(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

pub fn asteroid_rotation(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    asteroid_query.for_each_mut(|(mut transform, asteroid)| {
        let rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), asteroid.rotation * time.delta_seconds());
        transform.rotation = transform.rotation.mul_quat(rotation);
    });
}

pub fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    asteroid_query.for_each_mut(|(mut transform, asteroid)| {
        transform.translation.x += asteroid.velocity.x * time.delta_seconds();
        transform.translation.y += asteroid.velocity.y * time.delta_seconds();
    });
}

fn vec2_from_circle(angle: f32, radius: f32) -> Vec2 {
    Vec2::new(-angle.sin() * radius, angle.cos() * radius)
}
