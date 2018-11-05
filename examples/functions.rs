//rust中的函数和变量都是使用snake case规范风格
fn main() {
    another_function();
    print_value(5);

    println!("12 + 13 = {}", add(12, 13));
}

fn another_function() {
    println!("Another function invoke....");
}

//拥有参数的的函数
fn print_value(value: i32) {
    println!("The value is: {}", value);
}

//拥有返回值的函数
fn add(n: i32, m:i32) -> i32{
    return n + m;
}