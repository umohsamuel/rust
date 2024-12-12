// fn main() {

//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//         break counter * 2;
//         }
//     };

//     println!("the counter count is : {result}");
// }


// fn main(){
//     let mut count = 0;

//     'counting_loop: loop {
//         println!("count =  {count}");

//         let mut remaining = 10;

//        loop {
//         println!("remaining...  {remaining}");
//          if remaining == 9 {
//             break;
//         }

//         if count == 2 {
//             break 'counting_loop;
//         }

//         remaining -= 1;
//        }

//         count += 1;
//     }

//     println!("End count: {count}");
// }

fn main (){
    // let mut number = 3;

    // while number != 0 {
    //     println!("number is {number}");

    //     number -= 1;
    // }

    // println!("lift off");

    // let a = [10,20,30,40,50];

    // let mut index = 0;

    // while index < 5 {
    //     println!("value is: {}", {a[index]});

    //     index += 1;
    // }

    // for element in a {
    //     println!("value is: {}", element);
    // }

    for element in (1..4).rev() {
        println!("value is: {}", element);
    }
}