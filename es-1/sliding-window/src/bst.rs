use std::collections::BTreeSet;

pub fn bst(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();
    if k > n {
        return Vec::<i32>::new();
    }

    let mut maxs = Vec::with_capacity(n - k + 1);

    let mut set = BTreeSet::new();
    let mut max_sf = nums[0];

    for (i, &v) in nums.iter().enumerate() {
        set.insert((v, i));

        // keep track of the max so far to avoid a costly query to the set
        max_sf = max_sf.max(v);

        if i >= k {
            set.remove(&(nums[i - k], i - k));
            if max_sf == nums[i - k] {
                max_sf = set.last().unwrap().0;
            }
        }
        if i >= k - 1 {
            maxs.push(max_sf);
        }
    }

    maxs
}
