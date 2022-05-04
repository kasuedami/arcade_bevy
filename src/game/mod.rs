use bevy::{prelude::*, app::PluginGroupBuilder};

mod asteroids_game;
mod menu;
mod settings;
mod pause;

mod button_colors {
    use bevy::prelude::Color;

    pub(crate) const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub(crate) const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub(crate) const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(in self) enum GameState {
    Menu,
    Settings,
    Pause,
    Asteroids,
}

struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Menu);
    }
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(BasePlugin)
            .add(asteroids_game::AsteroidsPlugin)
            .add(menu::MenuPlugin)
            .add(pause::PausePlugin)
            .add(settings::SettingsPlugin);
    }
}
