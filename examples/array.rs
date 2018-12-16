fn main() {

    simple_array_example();

    multiple_array();
}



//简单一维数组
fn simple_array_example() {
    let a = [1, 2, 3, 4, 5];
    println!("a[2]={}", a[2]);

    let months = ["January", "February", "March", "April",
        "May", "June", "July", "August",
        "September", "October", "November", "December"];
    println!("months.length={}", months.len());

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    //数组遍历
    for element in &arr {
        println!("{:?}", element);
    }

    //内置的比较方法
    println!("{:?}", a < arr)
}

//多维数组
fn multiple_array() {
    //二维数组
    let v: [[i32; 2]; 3] = [[0, 0], [1, 0], [1, 1]];
    for i in &v{
        for a in i{
            print!("{:?}", a);
            print!("  ");
        }
        print!("\n")
    }
}