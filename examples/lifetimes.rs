
fn largest<'a, 'b>(str1: &'a str, str2: &'b str) -> &'b str
    where 'a : 'b {
    if str1.len() >= str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {

    let str1 = "123";
    {
        let str2 = "4567";
        let ret = largest(&str1, &str2);
        println!("str1={}", str1);
        println!("str2={}", str2);
        println!("ret={}", ret);
    }
}