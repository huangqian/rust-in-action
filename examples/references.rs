
//  引用是为了解决所有权传递的麻烦
fn main(){

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s value is '{}'", s)

    //可变引用有一个很大的限制：在特定的作用域中的特定数据有且只有一个可变引用

}

//s的作用域和所有权一样，是到函数末尾截止，不过引用离开作用后并不丢弃它指向的数据。
//将获取应用作为函数参数成为借用
fn calculate_length(s: &String) -> usize {
    s.len()
}

//借用和函数参数一样，都是不可变的，如果想修改引用指向的数据，需要使用mut修饰
fn change(some_string: &mut String){
    some_string.push_str(", world!");
}
