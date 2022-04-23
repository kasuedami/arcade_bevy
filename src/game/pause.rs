use bevy::prelude::*;
use std::time::Duration;
use crate::game::GameState;

const MINIMUM_TIME: Duration = Duration::from_millis(200);
pub const PAUSE_COOLDOWN: Duration = Duration::from_millis(200);

struct PauseEntered (Duration);
pub struct PauseExited (Duration);

#[derive(Component)]
struct PauseRoot;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Pause)
                    .with_system(on_enter))
            .add_system_set(
                SystemSet::on_update(GameState::Pause)
                    .with_system(handle_keyboard))
            .add_system_set(
                SystemSet::on_exit(GameState::Pause)
                    .with_system(on_exit));
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {

    commands.insert_resource(PauseEntered(time.time_since_startup()));

    let font: Handle<Font> = asset_server.load("fonts/Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
            ..Default::default()
        })
        .insert(PauseRoot);
}

fn handle_keyboard(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>, time: Res<Time>, entered: Res<PauseEntered>) {

    if time.time_since_startup() - entered.0 > MINIMUM_TIME {
        if keys.just_pressed(KeyCode::Escape) {
            game_state.pop().unwrap();
        }
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<PauseRoot>>, time: Res<Time>) {
    commands.entity(query.single()).despawn_recursive();
    commands.remove_resource::<PauseEntered>();
    commands.insert_resource(PauseExited(time.time_since_startup()));
}

pub(in crate::game) fn handle_start_pause(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    exited: Option<Res<PauseExited>>,
    time: Res<Time>
) {

    match exited {
        Some(exit_time) => {
            if time.time_since_startup() - exit_time.0 > PAUSE_COOLDOWN {
                commands.remove_resource::<PauseExited>();
            }
        },
        None => {
            if keys.just_pressed(KeyCode::Escape) {
                game_state.push(GameState::Pause).unwrap();
            }
        },
    }

}
