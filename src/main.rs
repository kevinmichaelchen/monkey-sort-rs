use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::env;

// TODO accept cli args
fn main() {

    let args: Vec<String> = env::args().collect();
    let default_size = 4;
    let mut end: i64 = default_size;
    if args.len() >= 2 {
        end = args[1].parse::<i64>().unwrap_or(default_size);
    }

    let mut rng = rand::thread_rng();
    let r = std::ops::Range { start: 0, end: end };
    let mut vec: Vec<i64> = r.map(|_| {
        // 1 (inclusive) to 21 (exclusive)
        rng.gen_range(1, 21)
    }).collect();

    println!("Sorting {:?}", vec);
    let i = monkey_sort(&mut vec);
    println!("Sorted result = {:?}. Took {} shuffles.", vec, i);
}

fn monkey_sort<T>(v: &mut Vec<T>) -> u64
    where
        T: Ord,
{
    let mut i: u64 = 0;
    loop {
        if is_sorted(v) {
            return i
        }
        let mut rng = thread_rng();
        v.shuffle(&mut rng);
        i = i + 1;
    }
}

fn is_sorted<T>(data: &[T]) -> bool
    where
        T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}
