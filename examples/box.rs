#[cfg(test)]
mod box_test {
    use box_test::List::{Cons, Nil};
    use MyBox;

    //box在堆上存储一个i32
    #[test]
    pub fn test_box_simple_demo() {
        let b = Box::new(5);
        println!("b = {}", b)
    }

    //box允许创建递归类型
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    //测试递归类型
    #[test]
    pub fn test_cons_list() {
        let list = Cons(1,
                        Box::new(Cons(2,
                                      Box::new(Cons(3,
                                                    Box::new(Nil))))));
    }


    //测试自定义智能指针
    #[test]
    pub fn test_mybox() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        println!("*y = {}", *y);
    }
}


//自定义智能指针
struct MyBox<T>(T);


impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

//通过实现Deref trait将某类型像引用一样处理

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}