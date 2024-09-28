use std::collections::BinaryHeap;

pub fn heap(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();

    if k > n {
        return Vec::<i32>::new();
    }

    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    for i in 0..k - 1 {
        heap.push((nums[i], i));
    }

    let mut maxs = Vec::with_capacity(n - k + 1);

    for i in k - 1..n {
        heap.push((nums[i], i));
        while let Some((_, idx)) = heap.peek() {
            if *idx < i - (k - 1) {
                heap.pop();
            } else {
                break;
            }
        }
        maxs.push(heap.peek().unwrap().0);
    }
    maxs
}
