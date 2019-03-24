//借用-borrow
//引用-reference
//指针-pointer
//这三个概念的例子

//----------------borrow--------------------
// 1. 所有权可以被借用(borrow)。
// 2. 借用指针的语法使用&符号或者&mut符号表示。&表示只读借用；&mut表示可读写借用。
// 3. 借用指针(borrow pointer)也可以称作为引用(reference)。
// 4. 借用指针与普通指针的内部数据一模一样，唯一的区别是语义层面上的。它的作用是告诉编译器，它对指向的这块区域没有所有权。
// 5. 对于&mut型指针，如果mut修饰的是变量名，那么它代表的这个变量可以被重新绑定；如果mut修饰的是"借用指针&"，那么它代表的是被指向的对象可以被修改



#[cfg(test)]
mod borrow_reference_pointer_test {


    //对于&mut型指针，如果mut修饰的是变量名，那么它代表的这个变量可以被重新绑定；
    // 如果mut修饰的是"借用指针&"，那么它代表的是被指向的对象可以被修改
    #[test]
    fn test_mut_borrow(){

        let mut var = 0_i32;

        {
            let p1 = &mut var; //p1指针本身不能被重新绑定，我们可以通过p1改变变量var的值。
            println!("before change *p1={}", *p1);
            *p1 = 1;
            println!("after change *p1={}", *p1);
        }

        {
            let temp = 2_i32;
            let mut p2 = &var;
            println!("*p2={}", *p2);

            //不能 *p2= temp，因为*p2是immutable的
        }
    }

}