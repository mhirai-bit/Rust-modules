fn main() {
    println!("Hello, world!");
    let numbers = 0..;
    let five_numbers = numbers.take(5);

    for number in five_numbers {
        println!("{}", number);
    }

    let ones = std::iter::repeat(1);
    let least = ones.min().unwrap();

    println!("The smallest number one is {}.", least);
}

