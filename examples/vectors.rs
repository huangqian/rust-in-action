//vector允许我们在一个单独的数据结构中存储多个值，
//它在内存中彼此相邻地排列所有的值。

fn main(){

    let mut v: Vec<i32> = Vec::new();
    //向vector中添加元素
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    //为了方便，Rust提供了vec!宏
    let v1 = vec![4, 5, 6];
    println!("{:?}", v1);

    //读取vector中的元素
    println!("v[1]={}", &v[1]);
    println!("v.get(1)={}", v.get(1).expect("Not Found value"));

    //遍历vector中的元素
    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }
    println!("v3={:?}", v3);

}