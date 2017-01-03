fn main() {
    let x = vec![1, 2, 3];
    let y = &x;
    println!("{:?}", x);
    println!("{:?}", y);
    let mut x_mut = vec![1, 2, 3];
    let y_mut = &mut x_mut;
    y_mut.push(4);
    println!("{:?}", y_mut);
    // println!("{:?}", x_mut);
    {
        let temp = vec![2, 3, 4];
        let x = vec![1, 2, 3];
        let mut y = &x;
        let mut z = &mut y;
        println!("{:?}", z);
        *z = &temp;
        println!("{:?}", z);
    }
    let mut x_scope = vec![1, 2, 3];
    {
        let y_scope = &mut x_scope;
        y_scope.push(4);
    }
    x_scope.push(5);
    println!("{:?}", x_scope);
    // let nope = {
    // let x = vec![1];
    // &x
    // };
}
