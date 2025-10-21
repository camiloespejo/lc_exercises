fn main() {
    println!("{}", divide(-2147483648, -3));
}

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor.overflowing_abs().1 && dividend.overflowing_abs().1 {
        return 1;
    }

    if divisor.overflowing_abs().1 {
        return 0;
    }

    if dividend.overflowing_abs().1 {
        if dividend.is_negative() {
            if dividend != i32::MIN || divisor == 1 {
                return i32::MIN;
            } else if divisor == -1 {
                return i32::MAX;
            }

            let mut aux = dividend;
            let original_divisor = divisor.abs();
            let mut divisor_string: String =
                original_divisor.to_string().chars().collect::<String>();
            divisor_string.push('0');
            let string_result = divisor_string.parse::<i32>();

            let mut adding_factor = 1;
            let mut elevated_divisor = match string_result {
                Ok(value) => {
                    adding_factor = 10;
                    value
                }
                Err(_) => original_divisor,
            };

            if elevated_divisor >= dividend {
                adding_factor = 1;
                elevated_divisor = original_divisor;
            }

            let val_divisor = if divisor.is_negative() {
                elevated_divisor
            } else {
                -elevated_divisor
            };

            let orig_val_divisor = if divisor.is_negative() {
                original_divisor
            } else {
                original_divisor
            };

            let divisor_abs = val_divisor.abs();
            let mut counter = 0;
            while (aux <= val_divisor && aux <= 0) || aux <= orig_val_divisor {
                if aux >= val_divisor && aux <= orig_val_divisor {
                    aux += orig_val_divisor;
                    counter += 1;
                } else {
                    aux += divisor_abs;
                    counter += adding_factor;
                }
            }

            if divisor.is_negative() {
                return counter;
            } else {
                return -counter;
            }
        } else {
            return i32::MAX;
        }
    }

    if divisor == 0 {
        return 0;
    }

    let mut aux = dividend.abs();
    let original_divisor = divisor.abs();
    let mut divisor_string: String = original_divisor.to_string().chars().collect::<String>();
    divisor_string.push('0');
    let string_result = divisor_string.parse::<i32>();

    let mut adding_factor = 1;
    let mut elevated_divisor = match string_result {
        Ok(value) => {
            adding_factor = 10;
            value
        }
        Err(_) => original_divisor,
    };

    if elevated_divisor >= dividend {
        adding_factor = 1;
        elevated_divisor = original_divisor;
    }

    let mut counter = 0;
    while aux >= elevated_divisor || aux >= original_divisor {
        if aux <= elevated_divisor && aux >= original_divisor {
            aux -= original_divisor;
            counter += 1;
        } else {
            aux -= elevated_divisor;
            counter += adding_factor;
        }
    }

    if dividend.is_negative() ^ divisor.is_negative() {
        counter = -counter;
    }

    counter
}
