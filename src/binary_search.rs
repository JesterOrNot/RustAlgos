use std::marker::Sized;
pub fn binary_search<T: Sized + PartialEq + PartialOrd>(array: &[T], target: T) -> bool {
    let mut low = 0;
    let mut high = array.len()-1;
    while low <= high {
        let mid = (low + (high - low)/2) as usize;
        if array[mid] == target {
            return true;
        } else if *array.get(mid).unwrap() < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    return false;
}
