enum Sentence {
    Hello(u32),
    Nice,
    Goodbye { Person: String },
}
fn manage(person: &Sentence) {
    match *person {
        Sentence::Hello(n) => {
            for _ in 0..n {
                println!("Hello");
            }
        }
        Sentence::Nice => {
            println!("Nice to meet you");
        }
        Sentence::Goodbye { Person: ref person } => {
            println!("say goodbye to {}", person);
        }
    }
}
fn main() {
    let p_vec =
        vec![Sentence::Hello(2), Sentence::Nice, Sentence::Goodbye { Person: "p_1".to_string() }];
    for index in 0..3 {
        manage(&p_vec[index]);
    }
}
