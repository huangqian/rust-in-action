
pub mod network;



#[cfg(test)]
mod tests{

    #[test]
    fn hello_test(){
        assert_eq!(1 + 1, 2);
        println!("hello test")
    }

    #[test]
    fn server_connect(){
        const CONNECT_SUCCESS: i32 = 1;
        use network::server::connect as server_connect;
        let status: i32 = server_connect();
        assert_eq!(status, CONNECT_SUCCESS);
    }
}
