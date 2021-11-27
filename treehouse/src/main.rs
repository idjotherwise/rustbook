#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("ifan", "Hello ifan, der mewn."),
        Visitor::new("dafydd", "Hi pony, the trombone is in the corner"),
        Visitor::new("johnston", "Hi Johnny, the pumpkin pie is on the tabble"),
    ];

    loop {
        println!("Hello, what's your name? Leave empty and press ENTER to quit.");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            // visitor is already on list
            Some(visitor) => visitor.greet_visitor(),
            // visitor is not on list, or has no name
            None => {
                if name.is_empty() {
                    // visitor has no name
                    break;
                } else {
                    // visitor has a name but is not on the list
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend."));
                }
            }
        }
    }
    println!("Final list of visitors:");
    println!("{:#?}", visitor_list);
}
