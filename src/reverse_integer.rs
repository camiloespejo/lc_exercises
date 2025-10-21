fn main() {
    let x = reverse(-123);
    println!("{}", x);
}

pub fn reverse(x: i32) -> i32 {
    let result = x.abs().to_string().chars().rev().collect::<String>();

    let n = result.parse::<i32>();

    match n {
        Ok(n) => {
            if x < 0 {
                return n * -1;
            }
            return n;
        }
        Err(_) => 0,
    }
}
