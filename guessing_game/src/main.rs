// io 라이브러리 사용 및 불러오는 방법
use std::io;

// rand 라이브러리 사용 시 (랜덤넘버 생성), Cargo.toml > dependencies 에 rand = 버전 입력 및 저장 후 cargo build 명령어 입력하면 rand 설치
use rand::Rng;

use std::cmp::Ordering;

use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is : {}", secret_number);

// 숫자가 아닌 타입을 입력하기 전까지 계속해서 실행하는 방식 (loop)
    loop {
        println!("Please input your guess.");

    let mut guess: String = String::new(); 

    io::stdin().read_line(&mut guess).expect("Failed to Read line");

//let guess: u32= guess.trim().parse().expect("Please type a number!");
let guess: u32= match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue
};
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".red()),
        Ordering::Equal => {
            println!("{}", "You win!".green());

            // break 넣었을 경우엔 loop 빠져나옴 
            break;
        }
    }
    }
}
