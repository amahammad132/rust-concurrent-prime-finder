use std::time::Instant;
use rayon::prelude::*;

const THREADS: i32 = 48;
const STARTING: i32 = 1;
const ENDING: i32 = 1000000;

fn is_prime(to_check: i32) -> bool {
    if to_check == 2 {
        return true
    }
    if to_check < 2 {
        return false
    }

    return if to_check % 2 != 0 {
        for i in (3..f64::floor(f64::sqrt(to_check as f64)) as i32).step_by(2) {
            if to_check % i == 0 {
                return false
            }
        }
        true
    } else {
        false
    }
}

fn prime_vec(start: i32, end: i32) -> Vec<i32> {
    let mut slice: Vec<i32> = Vec::new();
    for num in start..end {
        if is_prime(num) {
            slice.push(num);
        }
    }
    return slice;
}

fn main() {

    let start_time = Instant::now();
    let mut prime_pieces: Vec<Vec<i32>> = vec![Vec::new(); THREADS as usize];

    crossbeam::thread::scope(|scope| {
        //let mut threads = vec![];
        // let starting = 1;
        // let ending = 1000000;

        let mut input_start = 0;
        let mut input_end = 0;
        let end_val = 0;
        let interval = (ENDING - STARTING) / THREADS;

        for piece in prime_pieces.iter_mut() {
            input_start += end_val + STARTING;
            input_end += end_val + interval;
            scope.spawn(move |_| {
                // println!("Start val: {} End val: {}", input_start, input_end);
                *piece = prime_vec(input_start, input_end);
            });
            input_start = input_end;
        }

    }).unwrap();

    let mut num_of_primes = 0;
    for prime_piece in prime_pieces {
        num_of_primes += prime_piece.len();
        println!("{:?}", prime_piece);
    }
    println!("There are {} primes between {} and {}", num_of_primes, STARTING, ENDING);
    println!("Threads used: {}", THREADS);
    println!("Time taken: {}ms", start_time.elapsed().as_millis())
}
