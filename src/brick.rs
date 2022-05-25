use opengl_graphics::{Texture, TextureSettings};
use std::fmt;
use std::path::Path;

static BRICKS: [&str; 5] = [
    "/home/actuday/dev/arkanoid/assets/png/element_red_rectangle.png",
    "/home/actuday/dev/arkanoid/assets/png/element_blue_rectangle.png",
    "/home/actuday/dev/arkanoid/assets/png/element_green_rectangle_glossy.png",
    "/home/actuday/dev/arkanoid/assets/png/element_purple_rectangle_glossy.png",
    "/home/actuday/dev/arkanoid/assets/png/element_yellow_square_glossy.png",
];

pub struct Brick {
    texture: Texture,
    destroyed: bool,
    pub brick_sprite: graphics::Image,
    ball_colliding: bool,
}

impl fmt::Debug for Brick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Brick:")
    }
}

impl Brick {
    pub fn new(x_pos: f64, y_pos: f64) -> Self {
        // A texture to use with the image
        let texture = Texture::from_path(
            Path::new(BRICKS[rand::random::<usize>() % 5]),
            &TextureSettings::new(),
        )
        .unwrap();
        Brick {
            ball_colliding: false,
            texture,
            destroyed: false,
            brick_sprite: graphics::Image::new().rect([x_pos, y_pos, 30f64, 15f64]),
        }
    }

    /// Get a reference to the ball's texture.
    #[must_use]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Get the brick's destroyed.
    #[must_use]
    pub fn destroyed(&self) -> bool {
        self.destroyed
    }

    /// Set the brick's destroyed.
    pub fn set_destroyed(&mut self, destroyed: bool) {
        self.destroyed = destroyed;
    }

    /// Get the brick's ball colliding.
    #[must_use]
    pub fn ball_colliding(&self) -> bool {
        self.ball_colliding
    }

    /// Set the brick's ball colliding.
    pub fn set_ball_colliding(&mut self, ball_colliding: bool) {
        self.ball_colliding = ball_colliding;
    }
}
