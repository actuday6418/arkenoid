use graphics::color::WHITE;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, RenderArgs, RenderEvent, ResizeEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, Window};

mod ball;
mod brick;
mod player;
mod util;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
enum Direction {
    Left,
    Right,
}

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    glyph_cache: GlyphCache<'a>,
    player: player::Player,
    ball: ball::Ball,
    player_direction: Option<Direction>,
    window_height: f64,
    window_width: f64,
    player_sprite: graphics::Image,
    ball_sprite: graphics::Image,
    player_speed: f64,
    bricks: [brick::Brick; 100],
    score: u32,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            self.player_sprite.rectangle.as_mut().unwrap()[0] =
                self.player.x_offset() + self.window_width / 2f64;

            self.ball_sprite.rectangle.as_mut().unwrap()[0] = self.ball.x_pos();
            self.ball_sprite.rectangle.as_mut().unwrap()[1] = self.ball.y_pos();

            self.player_sprite.draw(
                self.player.texture(),
                &DrawState::new_alpha(),
                c.transform,
                gl,
            );
            self.ball_sprite.draw(
                self.ball.texture(),
                &DrawState::new_alpha(),
                c.transform,
                gl,
            );
            for brick in &self.bricks {
                if !brick.destroyed() {
                    brick.brick_sprite.draw(
                        brick.texture(),
                        &DrawState::new_alpha(),
                        c.transform,
                        gl,
                    );
                }
            }
            text::Text::new_color(WHITE, 33)
                .draw(
                    format!("Score: {}", self.score).as_str(),
                    &mut self.glyph_cache,
                    &c.draw_state,
                    c.transform.trans(25f64, 35f64),
                    gl,
                )
                .unwrap();
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Move ball one step in right direction
        self.ball.fly_tick();

        // if ball hits the horizontal walls
        if (self.ball.x_pos() < 5f64 || self.ball.x_pos() > self.window_width - 30f64)
            && !self.ball.wall_colliding()
        {
            self.ball.velocity[0] = -self.ball.velocity[0];
            self.ball.set_wall_colliding(true);
        } else if self.ball.wall_colliding() {
            self.ball.set_wall_colliding(false);
        }
        // if any of the bricks collide
        else if let Some(brick) = self.bricks.iter_mut().find_map(|x| {
            if !x.destroyed()
                && util::rectangles_collide(
                    x.brick_sprite.rectangle.as_ref().unwrap(),
                    self.ball_sprite.rectangle.as_ref().unwrap(),
                )
            {
                return Some(x);
            } else if x.ball_colliding() {
                if !util::rectangles_collide(
                    x.brick_sprite.rectangle.as_ref().unwrap(),
                    self.ball_sprite.rectangle.as_ref().unwrap(),
                ) {
                    self.ball.set_brick_colliding(false);
                    (x).set_ball_colliding(false);
                }
                None
            } else {
                None
            }
        }) {
            if !self.ball.brick_colliding() {
                self.score += 1;
                brick.set_destroyed(true);
                brick.set_ball_colliding(true);
                self.ball.velocity[1] *= -1f64;
                self.ball.set_brick_colliding(true);
            }
        }
        // if ball finishes colliding with a brick
        else if self.ball.brick_colliding() {
            self.ball.set_brick_colliding(false);
        }
        // if  ball hits the top or the paddle
        else if self.ball.y_pos() < 15f64
            || (util::rectangles_collide(
                self.ball_sprite.rectangle.as_ref().unwrap(),
                self.player_sprite.rectangle.as_ref().unwrap(),
            ) && !self.ball.colliding())
        {
            self.ball.velocity[1] *= -1f64;
            self.ball.set_colliding(true);
        }
        // if ball leaves the collision state with paddle
        else if !util::rectangles_collide(
            self.ball_sprite.rectangle.as_ref().unwrap(),
            self.player_sprite.rectangle.as_ref().unwrap(),
        ) && self.ball.colliding()
        {
            self.ball.set_colliding(false);
        // if ball falls off the screen
        } else if self.ball.y_pos() > self.window_height {
            self.reset();
        }
        if let Some(direction) = &self.player_direction {
            match direction {
                Direction::Left => {
                    if self.player.x_offset() > 5f64 - self.window_width / 2f64 {
                        self.player.add_x_offset(-self.player_speed);
                    }
                }
                Direction::Right => {
                    if self.player.x_offset() < self.window_width / 2f64 - 75f64 {
                        self.player.add_x_offset(self.player_speed);
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        self.player.set_x_offset(0f64);
        self.player_direction = None;
        self.ball.set_x_pos(self.window_width / 2f64);
        self.ball.set_y_pos(self.window_height / 3f64);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: glutin_window::GlutinWindow = WindowSettings::new("Arkenoid", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut bricks: Vec<brick::Brick> = Vec::new();
    let brick_width = window.size().width / 10f64;
    let brick_height = window.size().height / 10f64;
    for i in 0..10 {
        for j in 0..10 {
            bricks.push(brick::Brick::new(
                j as f64 * brick_width,
                i as f64 * brick_height,
            ));
        }
    }
    let bricks: [brick::Brick; 100] = bricks.try_into().unwrap();
    // Create a new game and run it.
    let mut app = App {
        score: 0,
        bricks,
        player_speed: 3f64 * window.size().width / window.size().height,
        gl: GlGraphics::new(opengl),
        player_direction: None,
        player_sprite: graphics::Image::new().rect([
            window.size().width / 2f64 - 35f64,
            6f64 * window.size().height / 7f64 - 7.5f64,
            70f64,
            15f64,
        ]),
        ball_sprite: graphics::Image::new().rect(graphics::rectangle::square(
            window.size().width / 2f64,
            2f64 * window.size().height / 3f64,
            20f64,
        )),
        glyph_cache: GlyphCache::new(
            "/home/actuday/dev/arkanoid/assets/Fonts/Kenney Future.ttf",
            (),
            TextureSettings::new(),
        )
        .unwrap(),
        player: player::Player::new(),
        ball: ball::Ball::new(window.size().width / 2f64, window.size().height / 3f64),
        window_height: window.size().height,
        window_width: window.size().width,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(args) = e.resize_args() {
            app.window_height = args.window_size[1];
            app.window_width = args.window_size[0];
            app.player_speed = 3f64 * args.window_size[0] / args.window_size[1];
            let brick_width = (window.size().width) / 10f64;
            let brick_height = window.size().height / 40f64;
            for i in 0..10 {
                for j in 0..10 {
                    app.bricks[i * 10 + j]
                        .brick_sprite
                        .rectangle
                        .as_mut()
                        .unwrap()
                        .copy_from_slice(&[
                            j as f64 * brick_width,
                            i as f64 * brick_height,
                            brick_width,
                            brick_height,
                        ]);
                }
            }
        }
        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(piston::Key::Right) => {
                    app.player_direction = Some(Direction::Right)
                }

                Button::Keyboard(piston::Key::Left) => app.player_direction = Some(Direction::Left),
                _ => {}
            }
        }
    }
}
