//HashMap

use std::collections::HashMap;

fn main(){

    //创建一个HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores={:?}", scores);

    //通过拉链操作创建HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let s2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("s2={:?}", s2);

}