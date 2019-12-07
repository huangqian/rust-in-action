#![feature(proc_macro_hygiene, decl_macro)]


extern crate redis;

use redis::Commands;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_hello_simple() {
        let client = redis::Client::open("redis://192.168.30.244/0").unwrap();
        let mut conn = client.get_connection().unwrap();

//        let _: () = redis::cmd("SET").arg("rust-key-1").arg(42).query(&conn).unwrap();

        let val: isize = redis::cmd("GET").arg("rust-key-1").query(&conn).unwrap();

        println!("rust-key-1 value was {}", val);
    }


    #[test]
    pub fn test_fetch_an_integer(){

        let val = fetch_an_integer().unwrap();
        println!("my-key value was {}", val);
    }


    pub fn fetch_an_integer() -> redis::RedisResult<isize>{

        //连接redis
        let client = redis::Client::open("redis://192.168.30.244/0")?;
        let conn = client.get_connection()?;
        let _ : ()  = conn.set("my-key", 42)?;
        conn.get("my-key")
    }

}
