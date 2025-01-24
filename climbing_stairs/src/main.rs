pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }
    let mut prev = 1;
    let mut curr = 2;
    for _ in 3..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn main() {
    let test_cases = vec![2, 3, 4, 5];

    for n in test_cases {
        println!("n = {}: {} ways", n, climb_stairs(n));
    }
}