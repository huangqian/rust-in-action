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

    let u4 = User{
        email: String::from("another@example.com"),
        //结构体更新语法，..语法制定了剩余未显示设置值的字段应与给定的实例对应的字段相同的值。
        ..u3
    };
    print_user(&u4);

    //元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    print_struct_example();

    associated_functions();
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

//元组结构体。元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
//当你想给整个元组取一个名字，并是元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

//方法和函数类似：它们使用fn关键字和名称声明，可以拥有参数与返回值，同时包含在某处调用该方法时会执行的代码。
//不过方法和函数是不同的，因为他们在结构体的上下文中被定义，并且它们第一参数总是self，它代表调用该方法的结构体实例

impl Rectangle{

    fn area(&self) -> u32{
        self.width * self.height
    }

    //多个参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    //关联函数，关联函数并不是实例的方法，它仍然是函数，它并不作用于一个结构体实例。
    //关联函数通过结构体和::调用，比如 Rectangle::square(10)
    fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }

}

fn print_struct_example(){
    let rect1 = Rectangle{width: 30, height: 50};
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn associated_functions(){
    let rect = Rectangle::square(10);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}