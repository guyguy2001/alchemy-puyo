use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{Pause, demo::level::spawn_level, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), scene.spawn());
}

fn scene() -> impl SceneList {
    bsn_list! {
        Name("foo")
        --
        @board()
    }
}

pub fn board() -> impl Scene {
    bsn! {
        Name("board")
        Mesh2d(asset_value(Rectangle::new(25.0, 25.0)))
    }
}
