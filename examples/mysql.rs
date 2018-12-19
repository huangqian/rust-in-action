
#[macro_use]
extern crate mysql;

use mysql::{Pool, Opts, OptsBuilder, from_value, Value};

#[derive(Debug, PartialEq, Eq)]
struct Payment{
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main(){

    let opts = get_mysql_opts();
    println!("{:?}", opts);
//    let pool = mysql::Pool::new(opts).unwrap();
//    pool.prep_exec(r"CREATE TEMPORARY TABLE payment (
//                         customer_id int not null,
//                         amount int not null,
//                         account_name text
//                     )", ()).unwrap();

}

fn get_mysql_opts() -> Opts{

    let user = "root";
    let pwd = "123456";
    let host = "127.0.0.1";
    let port = 3306;

    let mut builder = OptsBuilder::default()
        .user(Some(user))
        .pass(Some(pwd))
        .ip_or_hostname(Some(host))
        .tcp_port(port)
        .db_name(Some("test"))
        .init(vec!["SET GLOBAL sql_mode = 'TRADITIONAL'"]);

    builder.into()

}