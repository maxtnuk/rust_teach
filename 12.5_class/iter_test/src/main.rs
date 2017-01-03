fn main() {
    {
        let iter_v = vec![1, 2, 3, 4];
        for he in iter_v.iter() {
            println!("{}", he);
        }
    }
    {
        let mut mut_v = vec![1, 2, 3, 4];
        for he in mut_v.iter_mut() {
            *he += 1;
            println!("{}", he);
        }
        println!("{:?}", mut_v);
    }
    {
        let move_v = vec![1, 2, 3, 4];
        for he in move_v.into_iter() {
            println!("{}", he);
        }
        // println!("{:?}", move_v);
    }
}
