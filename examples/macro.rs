

//实现一个hashmap!宏
// let map = hashmap!["A" => 1, "B" => 2, "C" => 3];

#[macro_export]
macro_rules! hashmap {
 ( $($key: expr => $val: expr),*) =>{{
    let mut map = std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
 }};
}


fn main() {

    let index = hashmap!["A" => 0, "C" => 1, "G" => 20];
    println!("{:?}", index);
}


