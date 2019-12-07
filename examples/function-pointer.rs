

pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) ->i32{
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32{
    a + b
}


fn product(a: i32, b:i32) -> i32{
    a * b
}


fn is_true() -> bool { true}

fn true_maker() -> fn() -> bool {is_true}

fn main(){

    let a  = 2;
    let b  = 3;
    assert_eq!(math(sum, 2, 3), 5);
    assert_eq!(math(product, 2, 3), 6);
    assert_eq!(true_maker()(), true);

}