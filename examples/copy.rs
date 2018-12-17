
#[derive(Copy, Clone)]
struct Foo {
    data: i32
}



//impl Clone for Foo {
//    fn clone(&self) -> Foo {
//        Foo { data: self.data }
//    }
//}
//
//impl Copy for Foo {}

fn implement_var_copy() {

    let f1 = Foo{data : 1};
    let f2 = f1;
    //由于实现了Copy trait（Clone和Copy要一起实现），所以复制不是move，而是copy
    println!("{:?}", f1.data);
}

fn main() {
    let v1: isize = 1;
    let v2 = v1;
    println!("{}", v2);

    implement_var_copy();
}