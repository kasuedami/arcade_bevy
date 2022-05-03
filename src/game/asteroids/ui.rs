use bevy::prelude::*;

#[derive(Component)]
pub struct UiElement;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font_handle = asset_server.load("fonts/Regular.ttf");

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiElement);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(UiElement)
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

pub fn remove_ui(mut commands: Commands, query: Query<Entity, With<UiElement>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
