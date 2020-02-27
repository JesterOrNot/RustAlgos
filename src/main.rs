mod binary_search;
use binary_search::binary_search;
fn main() {
    let my_arr: [i32;3] = [1,2,3];
    println!("{}", binary_search(&my_arr, 5));
}