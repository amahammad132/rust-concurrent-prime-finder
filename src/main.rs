use std::time::{Instant, Duration};
use std::thread;
use std::thread::sleep;
use rayon::prelude::*;

const NTHREADS: i32 = 14;

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
    // println!("{:?}", slice);
    return slice;
}

fn main() {

    let start = Instant::now();
    /*
    //let mut threads = vec![];
    let starting = 1;
    let ending = 1000000;

    let mut input_start = 0;
    let mut input_end = 0;
    let end_val = 0;


    let interval = (ending - starting) / NTHREADS;
    //let mut primes: Vec<Vec<i32>> = vec![];
    let mut num_of_primes = 0;
    let mut aeae = 0;
    let mut primes: Vec<Vec<i32>> = Vec::new();
    //let mut array: [Vec<i32>; NTHREADS as usize] = [vec![]; NTHREADS as usize];
    //let mut testvec: aovec::Aovec<Vec<i32>> = aovec::Aovec::new(NTHREADS as usize);

    for i in 0..NTHREADS {
    //for (i, section) in &mut array.iter_mut().enumerate() {
    //let mut i = 0;
    //for section in array.iter_mut() {
        input_start += end_val + starting;
        input_end += end_val + interval;

        // makes sure all rounding errors are gone from splitting the intervals
        if i == NTHREADS - 1 {input_end = ending;}
        // Spin up another thread
        // threads.push(thread::spawn(move || {
        //     println!("RangePrime(start: {}, end: {})", input_start, input_end);
        //     testvec.push(prime_vec(input_start, input_end));
        //     //*section = prime_vec(input_start, input_end);
        // }));
        let var = 2;
        let thread_crossbeam = crossbeam::thread::scope(|s| {
            s.spawn(|_| {
                println!("RangePrime(start: {}, end: {})", input_start, input_end);
                primes.push(prime_vec(input_start, input_end));
            });
        });
        input_start = input_end;
        //i += 1;
    }

//    let conversion = testvec as Vec<Vec<i32>>;
    //let conversion: Vec<Vec<i32>> = Vec::from(testvec);

    for item in &primes {
        println!("{:#?}", item);
        num_of_primes += item.len();
    }
    // for child in threads {
    //     // Wait for the thread to finish. Returns a result.
    //     let _ = child.join();
    // }

    println!("There are {} primes between {} and {}", num_of_primes, starting, ending);


    */

     */

    static oae: i32 = 48;
    let mut prime_pieces: Vec<Vec<i32>> = vec![Vec::new(); oae as usize];

    crossbeam::thread::scope(|scope| {
        //let mut threads = vec![];
        let starting = 1;
        let ending = 1000000;

        let mut input_start = 0;
        let mut input_end = 0;
        let end_val = 0;


        let interval = (ending - starting) / oae;


        for mut piece in prime_pieces.iter_mut() {
            input_start += end_val + starting;
            input_end += end_val + interval;
            scope.spawn(move |_| {
                // person = &2;
                println!("Start val: {} End val: {}", input_start, input_end);
                println!("Time taken: {}us", start.elapsed().as_micros());
                *piece = prime_vec(input_start, input_end);
                //*person = vec![1, 3, 2, 1]
                //println!("Hello, {}!", person);

            });
            input_start = input_end;
        }

    }).unwrap();
    let mut suma = 0;
    for aea in prime_pieces {
        suma += aea.len();
        println!("{:?}", aea);
    }
    println!("The # of primes is: {}", suma);
    println!("Threads used: {}", oae);
    println!("Time taken: {}ms", start.elapsed().as_millis())
}
