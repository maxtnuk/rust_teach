extern crate Modul_test;

fn main() {
    println!("{}", Modul_test::english::greetings::hello());
    println!("{}", Modul_test::english::farewells::goodbye());

    println!("{}", Modul_test::japanese::greetings::hello());
    println!("{}", Modul_test::japanese::farewells::goodbye());
}
