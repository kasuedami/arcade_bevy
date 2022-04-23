use bevy::prelude::*;
use std::time::Duration;
use crate::game::GameState;

const MINIMUM_TIME: Duration = Duration::from_millis(200);
pub const PAUSE_COOLDOWN: Duration = Duration::from_millis(200);

struct PauseEntered (Duration);
pub struct PauseExited (Duration);

#[derive(Component)]
struct PauseRoot;

#[derive(Component, Clone, Copy)]
enum PauseButton {
    Continue,
    Quit,
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Pause)
                    .with_system(on_enter))
            .add_system_set(
                SystemSet::on_exit(GameState::Pause)
                    .with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::Pause)
                    .with_system(handle_keyboard)
                    .with_system(handle_buttons));
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {

    commands.insert_resource(PauseEntered(time.time_since_startup()));

    let font: Handle<Font> = asset_server.load("fonts/Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
            ..Default::default()
        })
        .insert(PauseRoot)
        .with_children(|mut parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Paused",
                        TextStyle {
                            font: font.clone(),
                            font_size: 80.0,
                            color: Color::rgb(0.9, 0.9, 0.9)
                        },
                        Default::default()
                    ),
                    style: Style {
                        margin: Rect::all(Val::Px(100.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            spawn_button(&mut parent, font.clone(), PauseButton::Continue);
            spawn_button(&mut parent, font.clone(), PauseButton::Quit);
        });
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

fn handle_buttons(
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<(&Interaction, &mut UiColor, &PauseButton)>,
) {

    query.for_each_mut(|(interaction, mut color, pause_button)| match interaction {
        Interaction::Clicked => {

            match pause_button {
                PauseButton::Continue =>
                    game_state.pop().unwrap(),
                PauseButton::Quit =>
                    game_state.replace(GameState::Menu).unwrap(),
            }

        }
        Interaction::Hovered => {
        }
        Interaction::None => {
        }
    });
}

fn spawn_button(commands: &mut ChildBuilder, font: Handle<Font>, button_type: PauseButton) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                margin: Rect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..Default::default()
        })
        .insert(button_type)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        match button_type {
                            PauseButton::Continue => "Continue",
                            PauseButton::Quit => "Quit",
                        },
                        TextStyle {
                            font: font.clone(),
                            font_size: 35.0,
                            color: Color::rgb(0.9, 0.9, 0.9)
                        },
                        Default::default()
                    ),
                    ..Default::default()
                });
        });
}
