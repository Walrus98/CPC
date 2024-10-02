pub fn search_range(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut maxs = Vec::<i32>::with_capacity(2);

    let mut left_range: i32 = -1;
    let mut right_range: i32 = -1;

    if nums.len() == 0 {
        maxs.push(left_range);
        maxs.push(right_range);
        return maxs;
    }

    let mut left_pos: usize = 0;
    let mut right_pos: usize = nums.len() - 1;

    while left_pos <= right_pos {
        let mid_pos: usize = left_pos + (right_pos - left_pos) / 2;
        let num = nums[mid_pos];

        if num == target {
            if nums[mid_pos - 1] != target {
                left_range = mid_pos as i32;
                left_pos = mid_pos + 1;
            }

            if nums[mid_pos + 1] != target {
                right_range = mid_pos as i32;
                right_pos = mid_pos - 1;
            }

            if left_range != -1 && right_range != -1 {
                break;
            }
        } else if num < target {
            left_pos = mid_pos + 1;
        } else {
            right_pos = mid_pos - 1;
        }
    }

    maxs.push(left_range);
    maxs.push(right_range);

    return maxs;
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let range: Vec<i32> = search_range(&nums, 8);
    println!("{:?}", range);
}
