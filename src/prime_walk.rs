use image::{Rgb, RgbImage};
use imageproc;
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Angle(f64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    Right,
    Left,
    Angle(f64),
}

// TODO: the current implementation is really two steps through the PrimeWalk iteration - the first with draw=false, the second =true.
// This is a terrible, horrible, no good, very bad implementation. It's not even a design. It's just the crap that I whipped up while
// organically testing this out.
// What should really be done is:
//  * Rewrite PrimeWalk to create a type that *only* processes primes to establish Cartesian bounds for what we will draw.
//  * Then, we either:
//     1. Create a second type that similarly processes primes using the output of type 1. This type would create the image and draw
//        lines as it processes input numbers. OR...
//     2. Aggregate non-normalized points in a Vec<(f32,f32)> as we process primes. When we're done iterating, we can then subtract
//        the min_y and min_y and scale as appropriate. Drawing the image is then just an iteration through this vector.
//
// The tradeoff is just the difference between storing a Vec in-memory (#2) or re-reading and -processing input (#1). As there are
//     50.8M primes < 1B, that's only 388Mb to keep in-memory. So that seems reasonable.
#[derive(Clone, PartialEq)]
pub struct PrimeWalk {
    steps_completed: u64,
    last_prime: i64,
    initial_direction: f64, // instead of using a Direction, store the angle. 0 is right, 90 is up (need to verify that i have the sign right here)
    turn: f64,              // how much to turn on each step; right is +90, Left is -90 or 270

    // current position
    x: f64,
    y: f64,
    direction: f64, // instead of using a Direction, store the angle. 0 is right, 90 is up (need to verify that i have the sign right here)

    // For the first pass - check output size
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,

    // For the second pass - drawing
    image: RgbImage,
    color: [u8; 3],
    scale: f64,
    draw: bool,
}

impl std::fmt::Debug for PrimeWalk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "PrimeWalk {{ steps_completed={}, last_prime={}, initial_direction={:?}, turn={:?}, xy = ({}, {}), direction={:?}, min_xy=({}, {}), max_xy=({}, {}) }}",
            self.steps_completed,
            self.last_prime,
            self.initial_direction,
            self.turn,
            self.x,
            self.y,
            self.direction,
            self.min_x,
            self.min_y,
            self.max_x,
            self.max_y,
        )
    }
}

impl PrimeWalk {
    const _WHITE: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
    const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
    const DEFAULT_SCALE: f64 = 25.;
    const SIZE: u32 = 5400;

    pub fn new() -> Self {
        let mut image = RgbImage::new(Self::SIZE, Self::SIZE);
        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(Self::SIZE, Self::SIZE),
            Self::BLACK,
        );

        let color = [255u8, 0, 0];

        Self {
            steps_completed: 0,
            last_prime: 1,
            x: 0.,
            y: 0.,
            initial_direction: 90., // 90 is north
            direction: 90.,         // 90 is north
            turn: 90.,
            min_x: 0.,
            min_y: 0.,
            max_x: 0.,
            max_y: 0.,

            image,
            color,
            scale: Self::DEFAULT_SCALE,
            draw: false,
        }
    }

    pub fn start_drawing(&mut self) {
        self.steps_completed = 0;
        self.last_prime = 1;
        self.x = 0. - self.min_x;
        self.y = 0. - self.min_y;
        self.direction = self.initial_direction;
        self.draw = true;

        println!(
            "Completed first step. Range was ({}, {}) to ({}, {})",
            self.min_x, self.min_y, self.max_x, self.max_y
        );
        println!(
            "    Width was {}, height was {}",
            self.max_x - self.min_x,
            self.max_y - self.min_y
        );
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        use Direction::*;
        self.direction = match direction {
            North => -90.,
            East => 0.,
            South => 90.,
            West => 180.,
            Angle(a) => a,
        };
        self.initial_direction = self.direction;

        self
    }

    pub fn with_turn(mut self, turn: Turn) -> Self {
        self.turn = match turn {
            Turn::Right => 90.,
            Turn::Left => -90.,
            Turn::Angle(a) => a,
        };

        assert!(self.turn < 360.);
        assert!(self.turn > -360.);
        assert!(self.turn != -180.);
        assert!(self.turn != 180.);

        self
    }

    pub fn with_start_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = [r, g, b];
        self
    }

    /// Determines the next position and heading for the given prime
    fn next_position(&self, prime: i64) -> (f64, f64, f64) {
        // using the current direction, move x and y
        let distance_traveled = (prime - self.last_prime) as f64;
        let rad = self.direction.to_radians();
        let move_x = distance_traveled * rad.cos();
        let move_y = distance_traveled * rad.sin();

        let new_direction = match self.direction + self.turn {
            angle if angle <= -360. => angle + 360.,
            angle if angle >= 360. => angle - 360.,
            angle => angle,
        };

        (self.x + move_x, self.y + move_y, new_direction)
    }

    pub fn advance_to_prime(&mut self, prime: i64) {
        let (new_x, new_y, new_direction) = self.next_position(prime);

        if self.draw {
            // Drawing - we need to shift the positions to be positive and scale to fit into the image
            let from_x = self.x / Self::DEFAULT_SCALE;
            let from_y = self.y / Self::DEFAULT_SCALE;

            let to_x = new_x / Self::DEFAULT_SCALE;
            let to_y = new_y / Self::DEFAULT_SCALE;

            draw_line_segment_mut(
                &mut self.image,
                (from_x as f32, from_y as f32),
                (to_x as f32, to_y as f32),
                Rgb(self.color),
            );

            // advance the color
            self.color = self.next_color();
        } else {
            // not drawing but keeping track of scale
            self.min_x = self.min_x.min(new_x);
            self.min_y = self.min_y.min(new_y);
            self.max_x = self.max_x.max(new_x);
            self.max_y = self.max_y.max(new_y);
        }

        self.steps_completed += 1;

        self.x = new_x;
        self.y = new_y;
        self.direction = new_direction;

        self.last_prime = prime;
    }

    /// Change the color to be used when drawing.
    ///
    /// This very simply treats RGB as a 24-bit number (red is higher, blue is lower), increments
    /// the value, and de-composes again.
    ///
    /// There's probably a better, more aesthetically pleasing way to change colors, but this was simple.
    fn next_color(&self) -> [u8; 3] {
        let r = self.color[0] as u32;
        let g = self.color[1] as u32;
        let b = self.color[2] as u32;

        let encoded = ((r << 16) | (g << 8) | b) + 1;

        // If we're at white (255, 255, 255), reset to black
        if encoded == 16777215 {
            [0, 0, 0]
        } else {
            let r = (encoded >> 16) & 255;
            let g = (encoded >> 8) & 255;
            let b = (encoded) & 255;

            [r as u8, g as u8, b as u8]
        }
    }

    /// Saves the image to the specified `path`.
    pub fn save_to(&self, path: &str) {
        self.image.save(path).unwrap();
    }
}
