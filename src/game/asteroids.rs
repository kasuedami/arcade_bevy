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
                    .with_system(on_enter))
            .add_system_set(
                SystemSet::on_exit(GameState::Asteroids)
                    .with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::Asteroids)
                    .with_system(on_update)
                    .with_system(handle_start_pause));
    }
}

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let mut ortho_camera = OrthographicCameraBundle::new_2d();
    ortho_camera.orthographic_projection.scale = 0.4;

    commands.spawn_bundle(ortho_camera);
    commands.spawn_bundle(UiCameraBundle::default());

    let fighter_mesh = create_fighter();
    let material = ColorMaterial {
        color: Color::WHITE,
        texture: None,
    };

    let fighter_handle = Mesh2dHandle(meshes.add(fighter_mesh));
    let material_handle = materials.add(material);

    commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: fighter_handle,
            material: material_handle,
            ..Default::default()
        })
        .insert(Player {
            velocity: Vec2::ZERO,
            rotation: 0.0,
        });
}

fn on_exit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroids_data: ResMut<AsteroidsData>,
    query: Query<Entity, With<Player>>
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

fn on_update(
    mut query: Query<(&mut Transform, &mut Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {

    let (mut transform, mut player) = query.single_mut();
    let mut rotated = false;
    let mut accelerated = false;

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

    let new_rot = transform.rotation.to_euler(EulerRot::ZYX);
    let direction_vec = Vec2::new(-new_rot.0.sin(), new_rot.0.cos());

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

    transform.translation.y += player.velocity.y;
    transform.translation.x += player.velocity.x;
}

fn pause_starter(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.push(GameState::Pause).unwrap();
    }
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
