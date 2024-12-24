fn main() {
    // #[derive(Debug)]
    // enum Directions {
    //     Up,
    //     Down,
    //     Left,
    //     Right,
    // }

    // let my_direction = Directions::Up;

    // #[derive(Debug)]
    // enum Directions {
    //     Up(String),
    //     Down(i32),
    //     Left(usize),
    //     Right(bool),
    // }

    // let my_direction = Directions::Up(String::from("North"));

    // println!("My direction {:?}", my_direction);

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let config = Option::Some(10);

    // match config {
    //     Option::Some(num) => println!("the number is {num}"),
    //     _ => (),
    // }

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    // can also be written as

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
