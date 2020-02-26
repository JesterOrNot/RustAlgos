use std::marker::Sized;
pub fn binary_search<T: Sized>(array: &[T]) -> T {
    return *array.get(0).unwrap()
}
