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

#[test]
pub fn test_binary_search_4() {
    let my_arr: [i32; 7] = [-66, 1, 2, 3, 5, 22, 100];
    assert_eq!(binary_search(&my_arr, -2000), false)
}

#[test]
pub fn test_binary_search_5() {
    let my_arr: [i32; 7] = [-66, 1, 2, 3, 5, 22, 100];
    assert_eq!(binary_search(&my_arr, 2000000), false)
}

#[test]
pub fn test_binary_search_6() {
    let my_arr: [i32; 22] = [
        -66, 1, 2, 3, 5, 22, 100, 3223, 323222, 40000000, 50000000, 60000000, 69000000, 70000000,
        80000000, 90000000, 99000000, 100000000, 110000000, 120000000, 200000000, 220000000,
    ];
    assert_eq!(binary_search(&my_arr, 220000000), true)
}
