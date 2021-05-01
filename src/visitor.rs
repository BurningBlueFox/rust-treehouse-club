use std::u8;

#[derive(Debug)]
pub struct Visitor {
    pub name: String,
    action: VisitorAction,
    pub age: u8,
}

impl Visitor {
    pub fn new(name: &str, action: VisitorAction, age: u8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    pub fn action(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse"),
            VisitorAction::AcceptWithNote { note } => {
                println!("{}", note);
                if &self.age < &21 {
                    println!("You are not allowed to deink alcohol on this club");
                }
            }
            VisitorAction::Probation => {
                &self.do_something_more_involved();
            }
            _ => println!("Go away!"),
        }
    }

    fn do_something_more_involved(&self) {
        println!("{} is now a new member on probation stage", &self.name);
    }
}

#[derive(Debug)]
pub enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
