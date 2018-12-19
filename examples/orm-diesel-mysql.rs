#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

//diesel是一个ORM框架
//这里是用diesel集合MySQL

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

table! {
posts(id) {
id -> Integer,
title -> Varchar,
body -> Text,
published -> Bool,
}
}


pub fn establish_connection() -> MysqlConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish()
}


fn main() {}