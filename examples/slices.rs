
fn main(){

    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear();
    println!(" match first key word index at: {}", word);

    let s1 = String::from("hello world");
    let hello = &s1[0..5];
    let world = &s1[6..11];
    println!("hello={}", hello);
    println!("world={}", world);
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}