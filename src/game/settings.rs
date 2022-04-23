use bevy::prelude::*;
use crate::game::{button_colors, GameState};

#[derive(Component)]
struct SettingsItem;

#[derive(Component, Clone, Copy)]
enum SettingsButton {
    NextSize,
    PreviousSize,
    Apply,
    Exit,
}

#[derive(Component, Clone, Copy)]
struct ScreenSizeDisplay;

#[derive(Clone, Copy)]
enum ScreenSize {
    Size1280x1024,
    Size1600x1200,
    Size1680x1050,
    Size1920x1080,
    Size1920x1200,
}

pub(crate) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Settings)
                    .with_system(spawn_ui)
                    .with_system(insert_resource)
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

fn insert_resource(mut commands: Commands) {
    commands.insert_resource(ScreenSize::Size1280x1024);
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
                                        size: Size::new(Val::Percent(100.0), Val::Px(180.0)),
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
                                                size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
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
                                        })
                                        .insert(ScreenSizeDisplay);

                                    spawn_button(parent, font_handle.clone(), SettingsButton::Apply);
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
    mut button_query: Query<(&Interaction, &mut UiColor, &SettingsButton), Changed<Interaction>>,
    mut display_text_query: Query<&mut Text, With<ScreenSizeDisplay>>,
    mut game_state: ResMut<State<GameState>>,
    mut screen_size: ResMut<ScreenSize>,
    mut windows: ResMut<Windows>
) {

    button_query.for_each_mut(|(interaction, mut color, menu_button)| match interaction {
        Interaction::Clicked => {

            match menu_button {
                SettingsButton::NextSize => {
                    *screen_size = next_screen_size(&screen_size);
                    let mut text = display_text_query.single_mut();
                    text.sections[0].value = text_for_screen_size(&*screen_size);
                },
                SettingsButton::PreviousSize => {
                    *screen_size = previous_screen_size(&screen_size);
                    let mut text = display_text_query.single_mut();
                    text.sections[0].value = text_for_screen_size(&*screen_size);
                },
                SettingsButton::Apply => {
                    let window = windows.get_primary_mut().unwrap();
                    let (width, height) = resoultion_for_screen_size(&screen_size);
                    window.set_resolution(width, height);
                },
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
                    SettingsButton::Apply => "Apply",
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

fn next_screen_size(screen_size: &ScreenSize) -> ScreenSize {
    match *screen_size {
        ScreenSize::Size1280x1024 => ScreenSize::Size1600x1200,
        ScreenSize::Size1600x1200 => ScreenSize::Size1680x1050,
        ScreenSize::Size1680x1050 => ScreenSize::Size1920x1080,
        ScreenSize::Size1920x1080 => ScreenSize::Size1920x1200,
        ScreenSize::Size1920x1200 => ScreenSize::Size1280x1024,
    }
}

fn previous_screen_size(screen_size: &ScreenSize) -> ScreenSize {
    match *screen_size {
        ScreenSize::Size1280x1024 => ScreenSize::Size1920x1200,
        ScreenSize::Size1600x1200 => ScreenSize::Size1280x1024,
        ScreenSize::Size1680x1050 => ScreenSize::Size1600x1200,
        ScreenSize::Size1920x1080 => ScreenSize::Size1680x1050,
        ScreenSize::Size1920x1200 => ScreenSize::Size1920x1080,
    }
}

fn resoultion_for_screen_size(screen_size: &ScreenSize) -> (f32, f32) {
    match *screen_size {
        ScreenSize::Size1280x1024 => (1280.0, 1024.0),
        ScreenSize::Size1600x1200 => (1600.0, 1200.0),
        ScreenSize::Size1680x1050 => (1680.0, 1050.0),
        ScreenSize::Size1920x1080 => (1920.0, 1080.0),
        ScreenSize::Size1920x1200 => (1920.0, 1200.0),
    }
}

fn text_for_screen_size(screen_size: &ScreenSize) -> String {
    match *screen_size {
        ScreenSize::Size1280x1024 => "1280.0 x 1024.0".to_string(),
        ScreenSize::Size1600x1200 => "1600.0 x 1200.0".to_string(),
        ScreenSize::Size1680x1050 => "1680.0 x 1050.0".to_string(),
        ScreenSize::Size1920x1080 => "1920.0 x 1080.0".to_string(),
        ScreenSize::Size1920x1200 => "1920.0 x 1200.0".to_string(),
    }
}
