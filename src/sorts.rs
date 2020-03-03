/// To search your the type of your array must implement: Sized PartialEq, PartialOrd
/// Time Complexity: O(log(n))
/// Space Complexity: O(1)
pub fn binary_search<T: Sized + PartialEq + PartialOrd>(array: &[T], target: T) -> bool {
    let mut low = 0;
    let mut high = array.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if array[mid] == target {
            return true;
        } else if array[mid] < target {
            low = mid + 1;
        } else if !(mid - 1 > 0) {
            high = mid - 1;
        } else {
            return false;
        }
    }
    return false;
}

pub fn selection_sort<T: Sized + PartialEq + PartialOrd>(list: &mut [T]) {
    for i in 0..list.len() {
        let mut small = i;
        for j in (i + 1)..list.len() {
            if list[j] < list[small] {
                small = j;
            }
        }
        list.swap(small, i);
    }
}
