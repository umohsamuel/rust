use std::io;

fn main() {
    let mut user_number = String::new();

    println!("Enter a number to calculate the fibonacci value: ");

    io::stdin()
        .read_line(&mut user_number)
        .expect("Failed to read line");

    let user_number: u32 = match user_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number, egbon");
            return;
        }
    };

    let final_value = fibonacci_sequence_nth_term(user_number);

    println!("fibonacci value for: {user_number} is: {final_value}")
}

fn fibonacci_sequence_nth_term(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;

        for _i in 2..=n {
            b += a;
            a = b - a;
        }
        return b;
    }
}
