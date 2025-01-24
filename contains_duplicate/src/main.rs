use std::collections::HashSet;

fn contains_duplicate(list: &[i32]) -> bool {
    let mut set = HashSet::new();
    for &item in list {
        if !set.insert(item) { // LEARN insert return false if elements already exists
            return true;
        }
    }
    false
}
fn main() {
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 2, 3, 4];

    println!("{}", contains_duplicate(&nums1)); // true
    println!("{}", contains_duplicate(&nums2)); // false
}