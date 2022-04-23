use bevy::{prelude::*, app::PluginGroupBuilder};

mod menu;
mod settings;
mod pause;
mod asteroids;

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
            .add(menu::MenuPlugin)
            .add(settings::SettingsPlugin)
            .add(pause::PausePlugin)
            .add(asteroids::AsteroidsPlugin);
    }
}
