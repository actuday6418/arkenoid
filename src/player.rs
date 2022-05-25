use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct Player {
    texture: Texture,
    x_offset: f64,
}

impl Player {
    pub fn new() -> Self {
        // A texture to use with the image
        let texture = Texture::from_path(
            Path::new("/home/actuday/dev/arkanoid/assets/png/paddleRed.png"),
            &TextureSettings::new(),
        )
        .unwrap();
        Player {
            texture,
            x_offset: 0f64,
        }
    }

    /// Get a reference to the player's texture.
    #[must_use]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Get the player's x offset.
    #[must_use]
    pub fn x_offset(&self) -> f64 {
        self.x_offset
    }

    /// Increment the player's x offset.
    pub fn add_x_offset(&mut self, count: f64) {
        self.x_offset += count;
    }

    /// Set the player's x offset.
    pub fn set_x_offset(&mut self, x_offset: f64) {
        self.x_offset = x_offset;
    }
}
