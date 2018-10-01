// 標準ライブラリのio。ユーザからの入力を受け付ける
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // ミュータブルなString変数
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
