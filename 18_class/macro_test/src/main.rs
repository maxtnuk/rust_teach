use std::collections::HashMap;
macro_rules! seperate{
    [$($x:expr),*] => {{
            let mut result= HashMap::new();
            $(
            let count=result.entry($x).or_insert(0);
            *count +=1;
            )*
            result
        }
    };
}
fn main() {
    let get = seperate![2, 1, 3, 4, 5, 2, 2, 9];
    println!("here is the result {:?}", get);
}
