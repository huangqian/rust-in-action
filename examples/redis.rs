#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate redis;

use redis::{Commands, PipelineCommands};

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

}
