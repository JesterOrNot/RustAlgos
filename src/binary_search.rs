/// To search your the type of your array must implement: Sized PartialEq, PartialOrd
pub fn binary_search<T: Sized + PartialEq + PartialOrd>(array: &[T], target: T) -> bool {
    let mut low = 0;
    let mut high = array.len() - 1;
    while low <= high {
        let mid: usize = low + (high - low) / 2;
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
