pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let num_digits = digits.len();

    num == digits.iter().map(|x| x.pow(num_digits as u32)).sum()
}

fn get_digits(mut num: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while num > 0 {
        result.push(num % 10);
        num /= 10;
    }

    result
}
