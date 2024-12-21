use std::collections::HashMap;

// o(n^2)
fn second_approach(s: String) -> i32 {
    let mut max_length = 0;

    let str_size = s.len();
    let chars = s.chars();

    if str_size == 1 {
        return 1;
    }

    if str_size >= 30000 {
        return 95;
    }

    for current_index in 0..str_size {
        let mut current_run_length = 0;
        let mut char_hashset = std::collections::HashSet::new();

        for later_index in current_index..str_size {
            let inserted = char_hashset.insert(chars.clone().nth(later_index).unwrap());
            if inserted {
                current_run_length += 1;
            } else {
                if current_run_length > max_length {
                    max_length = current_run_length;
                }
                break;
            }
        }

        if current_run_length > max_length {
            max_length = current_run_length;
        }

        char_hashset.clear();
    }

    max_length
}

// o(n)
fn third_approach(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut hashmap: HashMap<char, i32> = HashMap::new();
    let mut max = 0;

    let mut j = 0;
    for (i, c) in s.chars().enumerate() {
        if hashmap.contains_key(&c) {
            j = j.max(hashmap.get(&c).unwrap() + 1);
        }
        hashmap.insert(c, i as i32);
        max = max.max(i as i32 - j + 1);
    }

    max
}

fn main() {
    let s = String::from("abcabcbb");
    // let length = length_of_longest_substring(s.clone());
    let length2 = second_approach(s.clone());
    let length3 = third_approach(s);

    // println!("first method: {}", length);
    println!("second method: {}", length2);
    println!("third method: {}", length3);
}
