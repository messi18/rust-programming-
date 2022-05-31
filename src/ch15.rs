use std::ops::Deref;

struct MyBox<T> (T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl <T> MyBox<T> {
    fn new(t:T)-> MyBox<T> {
        MyBox(t)
    }
}
fn use_my_box() {
    let x = 1;
    let y = MyBox::new(x);
    println!("{}", *y);
}