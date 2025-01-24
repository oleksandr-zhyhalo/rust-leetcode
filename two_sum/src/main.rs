use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    nums.iter()
        .enumerate()
        .find_map(|(i, &v)| {
            let complement = target - v;
            if let Some(&j) = num_map.get(&complement) {
                Some(vec![j, i as i32])
            } else {
                num_map.insert(v, i as i32);
                None
            }
        })
        .unwrap_or_default()
}

fn main() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9),
        (vec![3, 2, 4], 6),
        (vec![3, 3], 6),
    ];

    for (nums, target) in test_cases {
        println!("nums: {:?}, target: {}, result: {:?}",
                 nums, target, two_sum(nums, target));
    }
}