fn main() {

    //通过为每个字段指定具体值来创建结构体的实例。
    //创建一个实例需要以结构体的名字开头，接着在大括号中使用key: value键-值对的形式提供字段；
    // 其中key是字段的名字，value是需要存储在字段中的数据值。
    let user1 = User {
        email: String::from("someone@gmail.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user1);

    let mut user2 = User{
        email: String::from("jerry@gamil.com"),
        username: String::from("jerry"),
        active: true,
        sign_in_count: 10,
    };

    print!("before change email: ");
    print_user(&user2);
    user2.email = String::from("jerry@163.com");
    print!("after change email: ");
    print_user(&user2);

    let email = String::from("huangqian866@163.com");
    let username = String::from("huangqian");
    let u3 = build_user(email, username);
    print_user(&u3);
}

//定义结构体需要使用struct关键字并为结构体提供一个名字。
struct User {
    //定义字段（field），每一个field包含名称和类型
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//构建User的函数
fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

//打印User的函数
fn print_user(user: &User){
    println!("username='{}', email='{}', active={}, sign_in_count={}",
             user.username,
             user.email,
             user.active,
             user.sign_in_count);
}

