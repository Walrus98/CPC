fn trapping_water(arr: &[i32]) -> i32 {
    let mut border_left: i32 = 0;
    let mut border_left_pos: i32 = 0;

    let mut border_right: i32;
    let mut border_right_pos: i32;

    let mut water_amount: i32 = 0;
    let mut exceeding_water: i32 = 0;

    // for (i: i32, &height) in arr.iter().enumerate() {
    for i in (0..n) {
        
        if arr[i] == 0 {
            continue;
        }

        if border_left == 0 {
            border_left = arr[i];
            border_left_pos = i;
            continue;
        }

        border_right = arr[i];
        border_right_pos = i;

        if border_left <= border_right {
            water_amount += (border_right_pos - border_left_pos - 1) * border_left;

            println!("{}", water_amount); 


            water_amount += exceeding_water;
            exceeding_water = 0;

            border_left = border_right;
        } else {
            water_amount += (border_right_pos - border_left_pos - 1) * border_right;
            exceeding_water += (border_right_pos - border_left_pos) * (border_left - border_right);
        }

        border_left_pos = border_right_pos;
    }

    water_amount
}

fn main() {
    let arr: Vec<i32> = vec![8, 8, 2, 4, 5, 5, 1];

    let result = trapping_water(&arr);
    
    println!("{}", result); 
}
