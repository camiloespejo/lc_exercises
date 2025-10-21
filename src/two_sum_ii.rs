use std::vec;

fn main() {
    let result = two_sum(vec![-1, 0], -1);

    println!("{:?}", result);
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, x) in numbers.iter().enumerate() {
        let result_search = numbers.binary_search(&(target - x));

        match result_search {
            Ok(x) => {
                if x != 0 {
                    return vec![index as i32 + 1, x as i32 + 1];
                }
                return vec![];
            }
            Err(_) => 0,
        };
    }

    vec![]
}
