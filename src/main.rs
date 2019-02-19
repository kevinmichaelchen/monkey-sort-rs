use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;

// TODO accept cli args
fn main() {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i64> = (0..4).map(|_| {
        // 1 (inclusive) to 21 (exclusive)
        rng.gen_range(1, 21)
    }).collect();

    println!("Sorting {:?}", vec);
    monkey_sort(&mut vec);
    println!("Sorted {:?}", vec);
}

fn monkey_sort<T>(v: &mut Vec<T>) -> &mut Vec<T>
    where
        T: Ord,
{
    monkey_sort_helper(v, 0)
}

fn monkey_sort_helper<T>(v: &mut Vec<T>, i: u64) -> &mut Vec<T>
    where
        T: Ord,
{
    println!("{}th shuffle", i);
    if is_sorted(v) {
        return v
    }
    let mut rng = thread_rng();
    v.shuffle(&mut rng);
    monkey_sort_helper(v, i + 1)
}

fn is_sorted<T>(data: &[T]) -> bool
    where
        T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}
