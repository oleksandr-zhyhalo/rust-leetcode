use std::collections::HashMap;
fn string_to_hm(s: &str) -> HashMap<char, i32> {
    let s1_hs  = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) +=1;
        map
    });
    s1_hs
}
fn valid_anagram(s1: &str, s2: &str) -> bool {
    let s1_hs  = string_to_hm(s1);
    let s2_hs  = string_to_hm(s2);
    if s1_hs == s2_hs
    {
        return true;
    }
    false
}
fn main() {
    let s1 = String::from("racecarer");
    let s2 = String::from("carraceed");
    let valid: bool = valid_anagram(&s1, &s2);
    println!("{valid}")
}
