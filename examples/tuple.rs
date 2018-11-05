
fn main(){

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess={}", guess);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={0}, y={1}, z={2}", x, y, z);
    println!("tup.0={0}, tup.1={1}, tup.2={2}", tup.0, tup.1, tup.2);
}

