struct Type_func<T> {
    have: T,
}
trait Print {
    fn print(&self);
}
impl<T> Type_func<T> {
    fn new(target: T) -> Self {
        Type_func { have: target }
    }
}
impl<T> Print for Type_func<T>
    where T: std::fmt::Display
{
    fn print(&self) {
        println!("{}", self.have);
    }
}
fn say_hello_too<T: Print>(x: T) {
    println!("hello!!");
    x.print()
}
fn main() {
    let u_32_test = Type_func::new(10u32);
    let str_test = Type_func::new("do your best");
    let string_test = Type_func::new("hell".to_string());
    say_hello_too(u_32_test);
    say_hello_too(str_test);
    say_hello_too(string_test);
}
