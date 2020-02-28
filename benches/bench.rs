#![feature(test)]

extern crate test;

use rust_algo::binary_search;
use test::Bencher;

#[bench]
pub fn bench_1(b: &mut Bencher) {
    b.iter(|| {
        let my_arr: [i32; 3] = [1, 2, 3];
        assert_eq!(binary_search(&my_arr, 3), true);
    })
}

#[bench]
fn bench_2(b: &mut Bencher) {
    b.iter(|| {
        let my_arr: [i32; 6] = [1, 2, 3, 5, 22, 100];
        assert_eq!(binary_search(&my_arr, 56), false);
    })
}

#[bench]
pub fn bench_3(b: &mut Bencher) {
    b.iter(|| {
        let my_arr: [i32; 6] = [1, 2, 3, 5, 22, 100];
        assert_eq!(binary_search(&my_arr, -2), false)
    })
}

#[bench]
pub fn bench_4(b: &mut Bencher) {
    b.iter(|| {
        let my_arr: [i32; 7] = [-66, 1, 2, 3, 5, 22, 100];
        assert_eq!(binary_search(&my_arr, -2000), false)
    })
}

#[bench]
pub fn bench_5(b: &mut Bencher) {
    b.iter(|| {
        let my_arr: [i32; 7] = [-66, 1, 2, 3, 5, 22, 100];
        assert_eq!(binary_search(&my_arr, 2000000), false)
    })
}

#[bench]
pub fn bench_6(b: &mut Bencher) {
    b.iter(|| {
        let my_arr: [i32; 22] = [
            -66, 1, 2, 3, 5, 22, 100, 3223, 323222, 40000000, 50000000, 60000000, 69000000,
            70000000, 80000000, 90000000, 99000000, 100000000, 110000000, 120000000, 200000000,
            220000000,
        ];
        assert_eq!(binary_search(&my_arr, 220000000), true)
    })
}
