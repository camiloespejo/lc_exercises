struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }

        let gap = num_rows - 2;
        let distance = num_rows + gap;
        let length = s.chars().count();
        let chars = s.chars().collect::<Vec<char>>();

        let mut new_distance = distance;
        let mut new_string: Vec<char> = Vec::new();
        let mut current_row = 0;
        let mut decrement = false;

        let mut is_whole = false;

        let mut lim = num_rows as usize - 1;
        let mut iter = 0..num_rows as usize;
        while let Some(x) = iter.next() {
            print!("[{}] ", x);
            if new_distance == 0 {
                new_distance = distance;
            }

            if num_rows + gap + x as i32 >= length as i32 {
                if current_row == num_rows - 1 {
                    decrement = true;
                } else if current_row == 0 {
                    decrement = false;
                }

                if decrement {
                    current_row -= 1;
                } else {
                    current_row += 1;
                }

                new_string.push(chars[x]);
                continue;
            }

            new_string.push(chars[x]);
            new_string.push(chars[x + new_distance as usize]);

            new_distance -= 2;

            if current_row == num_rows - 1 {
                decrement = true;
            } else if current_row == 0 {
                decrement = false;
            }

            if decrement {
                current_row -= 1;
            } else {
                current_row += 1;
            }

            if x == lim && !is_whole {
                lim = x + num_rows as usize; //?
                iter = (x + num_rows as usize)..lim + 1;

                println!("-> to long. st:{}, lim: {}", iter.start, lim);

                is_whole = true;
                current_row = 1;
                decrement = false;
                new_distance = distance - 2;
            } else if x == lim && is_whole {
                lim = (x + num_rows as usize) + (num_rows as usize - 1);
                iter = (x + num_rows as usize)..lim + 1;

                println!("-> to short. st:{}, lim: {}", iter.start, lim);

                is_whole = false;
                current_row = 0;
                decrement = false;
                new_distance = distance;
            }
        }

        new_string.iter().collect()
    }
}

fn main() {
    let result = Solution::convert("TRANSURFING".to_string(), 3);

    println!("\nresult: {}", result);
}
