mod primes;
use primes::PrimeIter;
mod prime_walk;
use prime_walk::{PrimeWalk, Turn};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    // To build an image with all primes under 10_000, run it as: ./prime-walk -p10000
    // Default will be to use values under 1M.
    let num_primes = match args.iter().find(|a| a.starts_with("-p")) {
        Some(a) => a[2..].parse().unwrap(),
        None => 1_000_000,
    };

    let mut walker = PrimeWalk::new().with_turn(Turn::Right);

    // Process data once to understand the size of the image, offset, etc
    for prime in PrimeIter::new(num_primes) {
        walker.advance_to_prime(prime);
    }

    // Then start the drawing
    walker.start_drawing();

    for prime in PrimeIter::new(num_primes) {
        walker.advance_to_prime(prime);
    }
    walker.save_to("primes.png");
}
