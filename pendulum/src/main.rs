use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use vector::Vector;

fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();
    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0)
    };
    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        self.p.draw(graphics);
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector,
    position: Vector,

    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,

    radius: f32,
    mass: f32,
    gravity: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            radius: radius,
            mass: 1.0,
            gravity: 1.5,
        }
    }

    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * (1.0 / self.mass) * self.gravity * self.angle.sin() / self.radius;
        //The angular velocity is the angular velocity plus the angular acceleration.
        self.angular_velocity += self.angular_acceleration;
        //The angle is the angle plus the angular velocity.
        self.angle += self.angular_velocity;
        //The posisition is the polar coordinates translated to cartesian coordinates.
        self.position.set(self.radius * self.angle.sin(), self.radius * self.angle.cos());
        //The final position of the ball in the canvas
        //pendulum plus the position vector.
        self.position.add(&self.origin);
    }

    fn draw(&mut self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::BLACK,
        );
        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLACK)
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Self {
            Self {
                x,
                y
            }
        }

        pub fn add(&mut self, other: &Vector) -> &Self{
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Self {
            self.x = x;
            self.y = y;
            self
        }
    }
}