mod brute_force;
mod bst;
mod heap;
mod linear;

fn main() {
    let v = vec![1, 3, 2, 5, 4, 8, 6];
    let k = 3;

    let result_brute_force = brute_force::brute_force(&v, k);
    println!("brute_force: {:?}", result_brute_force);
    
    let brute_force_elegant = brute_force::brute_force_elegant(&v, k);
    println!("brute_force: {:?}", brute_force_elegant);

    let result_bst = bst::bst(&v, k);
    println!("result_bst: {:?}", result_bst);

    let result_heap = heap::heap(&v, k);
    println!("result_heap: {:?}", result_heap);

    let result_linear = linear::linear(&v, k);
    println!("result_linear: {:?}", result_linear);
}
