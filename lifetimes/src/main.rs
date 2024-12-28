// fn main() {
//     let word = String::from("Hello, world!").to_string();

//     let kiniword = first_word(&word);

//     println!("first word is : {}", kiniword);
// }

// // fn first_word<'a>(s: &'a str) -> &'a str {
// //     s.split(",").next().unwrap()
// // }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// **** difficulty question

struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
