use bevy::prelude::*;
use crate::game::GameState;

#[derive(Component)]
struct SettingsItem;

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
                        size: Size::new(Val::Percent(50.0), Val::Percent(80.0)),
                        margin: Rect::all(Val::Px(100.0)),
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
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            color: Color::BLUE.into(),
                            ..Default::default()
                        });
                });
        });
}

fn remove_ui(mut commands: Commands, query: Query<Entity, With<SettingsItem>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
