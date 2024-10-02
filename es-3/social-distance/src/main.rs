use num::FromPrimitive;
use num::Num;
use std::cmp::PartialOrd;

fn binary_search_range<T, F>(low: T, high: T, pred: F) -> Option<T>
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: Fn(T) -> bool,
{
    let mut low = low;
    let mut high = high;

    let mut ans = None;

    while low < high {
        let middle = low + (high - low) / FromPrimitive::from_u64(2).unwrap();

        match pred(middle) {
            true => {
                low = middle + T::one();
                ans = Some(middle)
            }
            false => high = middle,
        }
    }

    ans
}

fn select_intervals(intervals: &mut Vec<(usize, usize)>, c: usize) -> Option<usize> {
    let l = intervals
        .iter()
        .fold(0, |acc, interval| acc + interval.1 - interval.0 + 1); // overall length

    if l < c {
        // there is no solution
        return None;
    }

    intervals.sort_unstable();

    // A closure implements our predicate
    let pred = |d: usize| -> bool {
        let mut last_selected = intervals[0].0;
        let mut cnt = 1;
        for &interval in intervals.iter() {
            while interval.0.max(last_selected + d) <= interval.1 {
                last_selected = interval.0.max(last_selected + d);
                cnt += 1;
            }
        }

        cnt >= c
    };

    binary_search_range(1, l + 1, pred)
}

fn main() {
    let mut intervals = vec![(0, 0), (8, 8)];

    let c = 2;

    match select_intervals(&mut intervals, c) {
        Some(distance) => println!("The optimal distance is: {}", distance),
        None => println!("No solution found."),
    }
}
