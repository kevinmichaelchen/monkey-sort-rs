use rand::thread_rng;
use rand::seq::SliceRandom;

// TODO accept cli args
fn main() {
    let mut vec = Vec::new();
    vec.push(4);
    vec.push(2);
    vec.push(1);
    vec.push(3);

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
    if is_sorted(v) {
        println!("Shuffed {} times", i);
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
