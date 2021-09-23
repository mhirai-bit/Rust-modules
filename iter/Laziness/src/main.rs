fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];
    v.iter().map(|x| println!("{}", x));

    v.iter().for_each(|x| println!("{}", x));

    for x in &v {
        println!("{}", x);
    }
}
