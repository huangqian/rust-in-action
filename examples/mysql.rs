#[macro_use]
extern crate mysql;

use mysql::{Pool, Opts, OptsBuilder};


#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn get_opts() -> Opts{

    let mut builder = OptsBuilder::default();
    builder.ip_or_hostname(Some("192.168.30.245"))
        .db_name(Some("test"))
        .user(Some("mysqlsiud"))
        .pass(Some("mysql!@#456"));
    builder.into()
}


fn main(){

    let opt = get_opts();

    let pool = Pool::new(opt).unwrap();

    let mut conn = pool.get_conn().unwrap();


    for r in conn.query("select * from lx_class").unwrap(){
        let mut r = r.unwrap();
        println!("{:?}", r)
    }

}


