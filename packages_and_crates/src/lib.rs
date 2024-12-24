mod add_numbers;

pub use crate::add_numbers::num_operation;

pub fn perform_operation() {
    let value = num_operation::add_2_numbers(3, 5);

    println!("the value of the computation is {}", value);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
