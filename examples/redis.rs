
extern  crate redis;

use redis::Commands;

fn main(){

    let val = get();
    println!("{:?}", val)
}


fn get() -> redis::RedisResult<isize> {

    let client = try!(redis::Client::open("redis://192.168.30.244/"));
    let con = try!(client.get_connection());

    let _ : () = try!(con.set("my_key", 42));

    con.get("my_key")
}