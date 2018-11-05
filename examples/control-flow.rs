fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
//if-else if - else if - else
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }
    //let 语句中使用if
    let val = if 16 % 4 == 0 {
        0
    } else {
        1
    };

    println!("val={}", val)
}

