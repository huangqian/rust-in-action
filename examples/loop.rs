
fn main(){

//    while_case();

//    while_demo_2();

    for_iterator();
}

fn while_case(){
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number -1;
    }
    println!("LIFTOFF!!!!")
}

fn while_demo_2(){

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_iterator(){

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}