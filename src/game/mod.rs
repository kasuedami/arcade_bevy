use bevy::{prelude::*, app::PluginGroupBuilder};

mod menu;
mod asteroids;

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
            .add(asteroids::AsteroidsPlugin);
    }
}
