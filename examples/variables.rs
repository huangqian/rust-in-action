
fn main(){

    //rust中默认的变量都是不可变的（immutable）。
    //在下面这个例子中，x的类型是推断出来的，x是不能再次被赋值
    let x = 5;
    println!(" The value of x is: {}", x);

    //可变变量也是非常有用的，如果声明一个可变变量，需要在声明的时候加上mut
    let mut v = 5;
    println!("The value of v is: {}", v);
    v = 6;
    println!("The value of v is: {}", v);



}