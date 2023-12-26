fn main() {
    let numbers = vec![5, 2, 8, 1, 2];
    let max_element = numbers.iter().max();

    match max_element {
        Some(&max) => println!("Maximum element: {}", max),
        None => println!("The vector is empty."),
    }
}
