use rand::prelude::*;
use rayon::prelude::*;

/// Coordinates type
type Coords = (f64, f64);

/// Size of one side of board
const SQUARE_SIZE: u8 = 2;

/// Half pf the board
const HALF: u8 = SQUARE_SIZE / 2;

/// Center of the board
const CENTER: Coords = (0.0, 0.0);

/// Amount of darts to throw
const DARTS: u128 = 1_000_000_000 as u128;

/// Number of experiments to run
const EXPERIMENTS: u64 = 10;


fn main() {
    println!("Running {} experiments...", EXPERIMENTS);
    let pi = average_experiment(EXPERIMENTS);
    println!("PI: {}", pi);
}

fn average_experiment(trials: u64) -> f64 {
    let total: f64 = (0..trials).into_par_iter().map(|i| {
        let pi = simulate_pi(DARTS);
        println!("Finished experiment #{}. Calculated Value: {}", i, pi);

        pi
    }).sum();

    total / trials as f64
}

/// Computes pi by throwing dart
fn simulate_pi(darts: u128) -> f64 {
    // TODO Rayon for parallel

    let inside: u128 = (0..darts).into_par_iter().map(|_| {
        let mut rng = rand::thread_rng();
        let (x, y) = throw_dart(&mut rng);

        if in_circle(x,y) {
            1
        }
        else {
            0
        }
    }).sum();

    4.0 * (inside as f64 / darts as f64)
}

/// Check whether a dart is inside the circle
fn in_circle(x: f64,y: f64) -> bool {
    let distance = (x.powi(2) + y.powi(2)).sqrt();

    if distance < SQUARE_SIZE as f64 / 2.0 {
        true
    }
    else {
        false
    }
}

fn throw_dart(rng: &mut ThreadRng) -> Coords {

    // Random
    let x_offset: f64 = HALF as f64 * (-1.0 + (2.0 * rng.gen::<f64>()));
    let y_offset: f64 = HALF as f64 * (-1.0 + (2.0 * rng.gen::<f64>()));

    (CENTER.0 + x_offset, CENTER.1 + y_offset)

}

mod tests {
    use super::*;

    #[test]
    /// Test the in_circle method
    fn inside(){
        assert!(in_circle(0.132, 0.378));
        assert!(!in_circle(SQUARE_SIZE as f64, 0.378));
        assert!(!in_circle(0.1234, SQUARE_SIZE as f64));
        assert!(!in_circle(SQUARE_SIZE as f64, SQUARE_SIZE as f64));
    }

    // #[test]
    // /// Test the in_circle method
    // fn inside(){
    // }

}
