
fn main(){

    let add = | a :i32, b:i32 | -> i32 { a + b};
    let x = add(1, 2);
    println!("result is {}", x);


    let add2 = | a, b | {a + b};
    println!("add2(1,2) = {}", add2(1, 2));

    //如果只有一条语句，{}可以省略

    let add3 = | a, b | a + b;

    println!("add3(1,2) = {}", add3(1, 2))
}

#[cfg(test)]
mod closure_test{


    #[test]
    fn test_simple_closure(){

        let add = |a, b| a + b;
        assert_eq!(3, add(1, 2));
        println!("add(1,2) = {}", add(1, 2))
    }
}