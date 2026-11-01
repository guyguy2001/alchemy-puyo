use bevy::{
    feathers::palette, image::TextureAtlasTemplate, input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::{Pause, demo::level::spawn_level, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), scene.spawn());
}

pub fn board() -> impl Scene {
    bsn! {
        Name("board")
        Transform::from_scale(Vec2::splat(8.0).extend(1.0))
        // Mesh2d(asset_value(Rectangle::new(25.0, 25.0)))
        // MeshMaterial2d<ColorMaterial>(asset_value(palette::GRAY_0))
        Children [
            @element(0)
            --
            @element(1)
            --
            @element(2)
            --
            @element(3)
        ]
    }
}

fn element(index: usize) -> impl Scene {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 4, None, None);
    bsn! {
        Transform::from_xyz(16. * index as f32, 0., 0.)
        Sprite {
            image: "images/elements.png",
            texture_atlas: {Some(TextureAtlasTemplate {
                layout: asset_value(layout),
                index,
            })}
        }
    }
}
