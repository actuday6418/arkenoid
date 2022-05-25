use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct Ball {
    texture: Texture,
    x_pos: f64,
    y_pos: f64,
    pub velocity: [f64; 2],
    colliding: bool,
    brick_colliding: bool,
    wall_colliding: bool,
}

impl Ball {
    pub fn new(x_pos: f64, y_pos: f64) -> Self {
        // A texture to use with the image
        let texture = Texture::from_path(
            Path::new("/home/actuday/dev/arkanoid/assets/png/ballBlue.png"),
            &TextureSettings::new(),
        )
        .unwrap();
        Ball {
            brick_colliding: false,
            wall_colliding: false,
            colliding: false,
            texture,
            x_pos,
            y_pos,
            velocity: [3f64; 2],
        }
    }

    /// Get a reference to the ball's texture.
    #[must_use]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn fly_tick(&mut self) {
        self.x_pos += self.velocity[0];
        self.y_pos += self.velocity[1];
    }

    /// Get the ball's x pos.
    #[must_use]
    pub fn x_pos(&self) -> f64 {
        self.x_pos
    }

    /// Get the ball's y pos.
    #[must_use]
    pub fn y_pos(&self) -> f64 {
        self.y_pos
    }

    /// Set the ball's x pos.
    pub fn set_x_pos(&mut self, x_pos: f64) {
        self.x_pos = x_pos;
    }

    /// Set the ball's y pos.
    pub fn set_y_pos(&mut self, y_pos: f64) {
        self.y_pos = y_pos;
    }

    /// Get the ball's colliding.
    #[must_use]
    pub fn colliding(&self) -> bool {
        self.colliding
    }

    /// Set the ball's colliding.
    pub fn set_colliding(&mut self, colliding: bool) {
        self.colliding = colliding;
    }

    /// Get the ball's brik colliding.
    #[must_use]
    pub fn brick_colliding(&self) -> bool {
        self.brick_colliding
    }

    /// Set the ball's brik colliding.
    pub fn set_brick_colliding(&mut self, brik_colliding: bool) {
        self.brick_colliding = brik_colliding;
    }

    /// Get the ball's wall colliding.
    #[must_use]
    pub fn wall_colliding(&self) -> bool {
        self.wall_colliding
    }

    /// Set the ball's wall colliding.
    pub fn set_wall_colliding(&mut self, wall_colliding: bool) {
        self.wall_colliding = wall_colliding;
    }
}
