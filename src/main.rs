use std::io::stdin;
mod visitor;
use visitor::{Visitor, VisitorAction};

fn main() {
    println!("Hello visitor, what is your name? \n");

    let mut visitor_list = vec![
        Visitor::new("thiago", VisitorAction::Accept, 25),
        Visitor::new(
            "john",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose free milk is in the fridge"),
            },
            12,
        ),
        Visitor::new("tyrone", VisitorAction::Refuse, 50),
    ];

    loop {
        let your_name = what_is_your_name();

        if your_name.is_empty() {
            break;
        }

        println!("\nHello {}!", your_name);
        let name_on_list = name_on_guestbook(&your_name, &visitor_list);

        if !name_on_list {
            visitor_list.push(Visitor::new(
                &your_name,
                visitor::VisitorAction::Probation,
                0,
            ));
        }
    }
    for i in 0..10 {
        println!("{}", i);
    }

    println!("The final visitor list: {:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Error reading line");
    your_name.trim().to_lowercase()
}

fn name_on_guestbook(name: &str, guestlist: &[Visitor]) -> bool {
    let known_visitor = guestlist.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => {
            visitor.action();
            return true;
        }
        None => {
            println!("{}", "You are not on the list");
            return false;
        }
    };
}
