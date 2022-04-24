use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::mesh::PrimitiveTopology;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
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

struct AsteroidsData {
    fighter_handle: Option<Mesh2dHandle>,
    material_handle: Option<Handle<ColorMaterial>>,
}

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AsteroidsData {
                fighter_handle: None,
                material_handle: None
            })
            .add_system_set(
                SystemSet::on_enter(GameState::Asteroids)
                    .with_system(spawn_player)
                    .with_system(spawn_background
                                 .before(spawn_player)
                    )
                    .with_system(spawn_ui)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Asteroids)
                    .with_system(on_exit)
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
                    .with_system(handle_start_pause)
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let mut ortho_camera = OrthographicCameraBundle::new_2d();
    ortho_camera.orthographic_projection.scale = 0.4;

    commands
        .spawn_bundle(ortho_camera)
            .insert(PlayerCamera);

    let fighter_mesh = create_fighter();
    let material = ColorMaterial {
        color: Color::GRAY,
        texture: None,
    };

    let fighter_handle = Mesh2dHandle(meshes.add(fighter_mesh));
    let material_handle = materials.add(material);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: fighter_handle,
            material: material_handle,
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

fn on_exit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroids_data: ResMut<AsteroidsData>,
    query: Query<Entity, With<AsteroidsItem>>
) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });

    let fighter_handle = &asteroids_data.fighter_handle;
    let material_handle = &asteroids_data.material_handle;

    match fighter_handle {
        Some(handle) => {
            meshes.remove(&handle.0);
        }
        None => {},
    }

    match material_handle {
        Some(handle) => {
            materials.remove(handle);
        }
        None => {},
    }

    asteroids_data.fighter_handle = None;
    asteroids_data.material_handle = None;
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

    println!("{}", player_transform.translation);

    camera_transform.translation.x += correction.x;
    camera_transform.translation.y += correction.y;
}

fn create_fighter() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(4);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(4);
    let mut uv: Vec<[f32; 2]> = Vec::with_capacity(4);
    let mut indices: Vec<u32> = Vec::with_capacity(5);

    positions.push([ 0.0,   25.0, 0.0]);
    positions.push([ 20.0, -25.0, 0.0]);
    positions.push([ 0.0,  -15.0, 0.0]);
    positions.push([-20.0, -25.0, 0.0]);

    normals.push([0.0, 0.0, 1.0]);
    normals.push([0.0, 0.0, 1.0]);
    normals.push([0.0, 0.0, 1.0]);
    normals.push([0.0, 0.0, 1.0]);

    uv.push([0.0, 0.0]);
    uv.push([0.0, 0.0]);
    uv.push([0.0, 0.0]);
    uv.push([0.0, 0.0]);

    indices.push(0);
    indices.push(1);
    indices.push(2);
    indices.push(3);
    indices.push(0);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
