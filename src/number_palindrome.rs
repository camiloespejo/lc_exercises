fn main() {
    let result = is_palindrome(10);

    println!("{}", result);
}

pub fn best_solution(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    return s == s.chars().rev().collect::<String>();
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let number_string = x.to_string();
    let mut chars = number_string.chars();
    let mut chars_rev = number_string.chars().rev();
    let string_length = number_string.len();

    let mut index = 0;
    while index < string_length / 2 {
        if chars.next() != chars_rev.next() {
            return false;
        }
        index += 1;
    }

    true
}
