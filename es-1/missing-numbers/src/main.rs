pub fn missing_number(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    for &num in nums {
        sum -= num;
    }

    sum 
}

fn main() {
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    let missing = Solution::missing_number(&nums);
    println!("Il numero mancante è: {}", missing); // Output: 8
}
