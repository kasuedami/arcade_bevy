use bevy::prelude::*;
use crate::game::GameState;
use crate::game::pause::handle_start_pause;

mod asteroids;
mod background;
mod player;
mod ui;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Asteroids)
                    .with_system(asteroids::asteroids_setup)
                    .with_system(player::spawn_player)
                    .with_system(background::spawn_background)
                    .with_system(ui::spawn_ui)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Asteroids)
                    .with_system(ui::remove_ui)
                    .with_system(player::remove_player)
                    .with_system(asteroids::remove_asteroids_atlas)
                    .with_system(asteroids::remove_asteroids
                                 .before(asteroids::remove_asteroids_atlas)
                    )
            )
            .add_system_set(
                SystemSet::on_update(GameState::Asteroids)
                    .with_system(player::acceleration)
                    .with_system(player::rotation
                                 .after(player::acceleration)
                    )
                    .with_system(player::camera_follow
                                 .after(player::acceleration)
                    )
                    .with_system(player::player_shoot_laser
                                 .after(player::rotation)
                    )
                    .with_system(player::laser_movement)
                    .with_system(player::laser_despawner
                                 .after(player::laser_movement)
                    )
                    .with_system(asteroids::asteroid_number_timer)
                    .with_system(asteroids::spawn_asteroid
                                 .after(asteroids::asteroid_number_timer)
                    )
                    .with_system(asteroids::asteroid_rotation)
                    .with_system(asteroids::asteroid_movement)
                    .with_system(asteroids::asteroid_distance_cleanup)
                    .with_system(handle_start_pause)
            );
    }
}
