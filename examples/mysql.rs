extern crate core;
#[macro_use]
extern crate mysql;

use mysql::{from_value, Opts, OptsBuilder, Pool, Value};

#[cfg(test)]
mod mysql_test {
    use std::fmt::{Display, Error, Formatter};

    use mysql::OptsBuilder;

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Payment {
        customer_id: i32,
        amount: i32,
        account_name: Option<String>,
    }

    impl Display for Payment {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            match &self.account_name {
                Some(an) => write!(f, "customer_id={}, amount={}, account_name={}",
                                   self.customer_id, self.amount, an),
                None => write!(f, "customer_id={}, amount={}, account_name=None",
                               self.customer_id, self.amount),
            }
        }
    }

    fn get_mysql_opts_builder() -> OptsBuilder {
        let mut builder = mysql::OptsBuilder::default();
        builder.user(Some("root".to_string()))
            .pass(Some("123456".to_string()))
            .ip_or_hostname(Some("127.0.0.1"))
            .tcp_port(3306)
            .db_name(Some("rust_example"));
        builder
    }

    #[test]
    pub fn test_insert_to_mysql() {
        let connection_url = "mysql://root:123456@localhost:3306/rust_example?prefer_socket=false";

        let pool = mysql::Pool::new(connection_url).unwrap();

        let payments = vec![
            Payment { customer_id: 0, amount: 2, account_name: None },
            Payment { customer_id: 0, amount: 4, account_name: Some("foo".into()) },
            Payment { customer_id: 0, amount: 6, account_name: None },
            Payment { customer_id: 0, amount: 8, account_name: None },
            Payment { customer_id: 0, amount: 10, account_name: Some("bar".into()) },
        ];

        for mut stmt in pool.prepare("insert into payment(amount, account_name) values (:amount, :account_name)")
            .into_iter() {
            for p in payments.iter() {
                stmt.execute(params! {
                "amount" => p.amount,
                "account_name" => &p.account_name,
                }).unwrap();
            }
        }
    }

    #[test]
    pub fn test_select_all() {
        let pool = mysql::Pool::new_manual(1, 2, get_mysql_opts_builder()).unwrap();
        let selected_payments: Vec<Payment> = pool.prep_exec("select * from payment", ())
            .map(|result| {
                result.map(|x| x.unwrap())
                    .map(|row| {
                        let (customer_id, amount, account_name) = mysql::from_row(row);
                        Payment {
                            customer_id: customer_id,
                            amount: amount,
                            account_name: account_name,
                        }
                    }).collect()
            }).unwrap();

        for p in selected_payments.iter() {
            println!("{:?}", *p);
        }
    }
}