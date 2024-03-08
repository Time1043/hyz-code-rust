use std::num::ParseIntError;

fn main() {
    let result = square("ZH"); // Err(ParseIntError { kind: InvalidDigit })
    println!("{:?}", result);
}

fn square(val: &str) -> Result<i32, ParseIntError> {
    let num = val.parse::<i32>()?; // 可能出错的表达式 ?
    Ok(num ^ 2)
}
