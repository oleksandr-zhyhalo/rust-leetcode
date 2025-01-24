pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let digit_array: Vec<char> = x.to_string().chars().collect();
    let len = digit_array.len();

    for i in 0..len/2 {
        if digit_array[i] != digit_array[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let test_cases = vec![121, -121, 10, 12321, 12345, 11];

    for num in test_cases {
        println!("Is {} a palindrome? {}", num, is_palindrome(num));
    }
}