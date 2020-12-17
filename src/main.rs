use quicksilver::prelude::*;

mod game;
pub use crate::game::Game;

// mod tile;
// pub use crate::tile::Tile;

fn main() {
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };

    run::<Game>("Quicksilver Rouguelike", Vector::new(800, 600), settings);
}
