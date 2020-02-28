use rust_algo::binary_search;
#[test]
pub fn test_binary_search_1() {
    let my_arr: [i32; 3] = [1, 2, 3];
    assert_eq!(binary_search(&my_arr, 3), true);
}

#[test]
pub fn test_binary_search_2() {
    let my_arr: [i32; 6] = [1, 2, 3, 5, 22, 100];
    assert_eq!(binary_search(&my_arr, 56), false);
}

#[test]
pub fn test_binary_search_3() {
    let my_arr: [i32; 6] = [1, 2, 3, 5, 22, 100];
    assert_eq!(binary_search(&my_arr, -2), false)
}
