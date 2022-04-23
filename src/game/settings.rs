use bevy::prelude::*;
use crate::game::{button_colors, GameState};

#[derive(Component)]
struct SettingsItem;

#[derive(Component, Clone, Copy)]
enum SettingsButton {
    NextSize,
    PreviousSize,
    Exit,
}

pub(crate) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Settings)
                    .with_system(spawn_ui)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Settings)
                    .with_system(remove_ui)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Settings)
                    .with_system(handle_buttons)
            );
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font_handle = asset_server.load("fonts/Regular.ttf");

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(SettingsItem);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            color: Color::rgb(0.9, 0.9, 0.9).into(),
            ..Default::default()
        })
        .insert(SettingsItem)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(70.0)),
                        margin: Rect::all(Val::Px(50.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
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
                                "Settings",
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 60.0,
                                    color: Color::rgb(0.2, 0.2, 0.2)
                                },
                                Default::default()
                            ),
                            ..Default::default()
                        });
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(80.0)),
                                margin: Rect::all(Val::Px(30.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Baseline,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Px(120.0)),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexStart,
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Px(66.0)),
                                                justify_content: JustifyContent::FlexStart,
                                                ..Default::default()
                                            },
                                            color: Color::NONE.into(),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            spawn_button(parent, font_handle.clone(), SettingsButton::PreviousSize);
                                            spawn_button(parent, font_handle.clone(), SettingsButton::NextSize);
                                        });
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Screen size",
                                                TextStyle {
                                                    font: font_handle.clone(),
                                                    font_size: 35.0,
                                                    color: Color::rgb(0.5, 0.5, 0.5)
                                                },
                                                Default::default()
                                            ),
                                            style: Style {
                                                margin: Rect::all(Val::Px(10.0)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        });
                                });

                            spawn_button(parent, font_handle.clone(), SettingsButton::Exit);
                        });
                });
        });
}

fn remove_ui(mut commands: Commands, query: Query<Entity, With<SettingsItem>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

fn handle_buttons(
    mut query: Query<(&Interaction, &mut UiColor, &SettingsButton)>,
    mut game_state: ResMut<State<GameState>>
) {

    query.for_each_mut(|(interaction, mut color, menu_button)| match interaction {
        Interaction::Clicked => {

            match menu_button {
                SettingsButton::NextSize => {},
                SettingsButton::PreviousSize => {},
                SettingsButton::Exit => {
                    game_state.set(GameState::Menu).unwrap();
                },
            }

            *color = button_colors::NORMAL_BUTTON.into();
        }
        Interaction::Hovered => {
            *color = button_colors::HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = button_colors::NORMAL_BUTTON.into();
        }
    });
}

fn spawn_button(commands: &mut ChildBuilder, font: Handle<Font>, button_type: SettingsButton) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(100.0), Val::Px(50.0)),
            margin: Rect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: button_colors::NORMAL_BUTTON.into(),
        ..Default::default()
    })
    .insert(button_type)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                match button_type {
                    SettingsButton::NextSize => "=>",
                    SettingsButton::PreviousSize => "<=",
                    SettingsButton::Exit => "Exit",
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
