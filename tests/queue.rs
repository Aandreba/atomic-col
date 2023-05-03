#![cfg(feature = "alloc")]

use rand::random;
use std::{
    thread::{available_parallelism, sleep, spawn},
    time::Duration,
};
use utils_atomics::*;

const RUNS: usize = 10;
const STRESS: i32 = 50;

#[test]
fn stress_fill_queue() {
    static QUEUE: FillQueue<i32> = FillQueue::new();

    for _ in 1..available_parallelism().unwrap().get() {
        spawn(move || loop {
            let v = random::<i32>();
            QUEUE.push(v);

            let nanos = i32::abs(v / (2 * STRESS));
            sleep(Duration::from_nanos(nanos as u64));
        });
    }

    for _ in 0..RUNS {
        sleep(Duration::from_secs(1));
        let count = QUEUE.chop().count();
        println!("Chopped elements: {count}!")
    }
}
