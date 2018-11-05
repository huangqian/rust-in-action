extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    game();

}


/*
 猜数字的游戏
*/
fn game(){
    println!("Gues the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

//-----------------------------------------------------
// struct example
//----------------------------------------------------

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_example(){
    let user1 = User{
        email: String::from("huangqian@weli.cn"),
        username: String::from("huangqian"),
        active: true,
        sign_in_count: 1,
    };

}