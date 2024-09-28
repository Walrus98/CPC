pub fn leaders(arr: &[i32]) -> Vec<i32> {
    let n: usize = arr.len();
    let mut leaders_array: Vec<i32> = Vec::new();

    let mut max: i32 = i32::MIN;

    for i in (0..n).rev() {
        if arr[i] > max {
            leaders_array.push(arr[i]);
            max = arr[i];
        }
    }

    leaders_array
}

fn main() {
    let arr: Vec<i32> = vec![16, 17, 4, 3, 5, 2];
    let leaders: Vec<i32> = leaders(&arr);
    println!("{:?}", leaders); // Output will be [17, 5, 2]
}
