use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> String {
    let mut hashmap_chars: HashMap<char, usize> = HashMap::new();

    let mut target_pos = 0;
    for (i, char) in s.chars().enumerate() {
        // target_pos = i;
        let present = hashmap_chars.insert(char, i);

        if present.is_some() {}

        println!("{}", char);
    }

    return String::from("x");
}

fn main() {
    print!("x");
}
