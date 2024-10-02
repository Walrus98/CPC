use std::collections::VecDeque;

pub fn linear(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();

    if k > n {
        return Vec::<i32>::new();
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);

    for i in 0..k {
        while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    maxs.push(nums[*q.front().unwrap()]);

    for i in k..n {
        while !q.is_empty() && q.front().unwrap() + k <= i {
            // more idiomatic while let Some(&(p,_)) = q.front()
            q.pop_front();
        }

        while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
        maxs.push(nums[*q.front().unwrap()]);
    }
    maxs
}
