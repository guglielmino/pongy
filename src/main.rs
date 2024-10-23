use bevy::prelude::*;
use pongy::{spawn_camera, spawn_players, spawn_ball, move_ball, move_paddle, ball_collide};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
    app.add_systems(Update, (move_ball, move_paddle, ball_collide));

    app.run();
}

