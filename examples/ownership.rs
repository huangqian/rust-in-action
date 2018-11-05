
//每一个值只有一个所有权的变量

fn main(){

    let s1 = String::from("hello");
    //将s1 move 给了s2, 这个时候s1没有用了，所有权也转交给了s2
    let s2 = s1;
    //s1没有了所有权，因此无法获取到任何值，这里调用是错误的，只能用s2
//    println!("{}, world", s1)
}