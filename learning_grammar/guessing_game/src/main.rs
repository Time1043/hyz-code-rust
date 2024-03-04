use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude // trait

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101); // i32 u32 u64
    println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜数!!!");
        println!("猜测一个数!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // io::Result  Ok, Err
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数字是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too samll!"), //arm
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
