use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}



fn main() {
    loop{
    println!("Hello, what's your name ? (Leave empty and press ENTER to quit)");

    let name = what_is_your_name();

    // println!("hello, {:?}", name);

    let mut visitor_list = vec![
        Visitor::new("Ajayi", VisitorAction::Accept, 45),
        Visitor::new("Damola", VisitorAction::AcceptWithNote{
            note: "the milk is inside the fridge".to_string()
        }, 15),
        Visitor::new(
            "Ramon",
            VisitorAction::Refuse, 30
        ),
    ]; /*an array of visitors */

    let known_visitor =  visitor_list.iter().find(|visitor| visitor.name == name);

    
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => {
            if name.is_empty(){
                break;
            }else {
                println!("{} is not in the list of visitors", name);
                 visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0))
            }
        }
    }

    println!("{:#?}", visitor_list)
    }
    // let mut allow_them_in = false;

    // for visitor in &visitor_list {
    //     if visitor == &name {
    //         allow_them_in = true;

    //     }
    //   }

    // if allow_them_in  {
    //     println!("Welcome to the Treehouse, {}", &name);
    // } else {
    //     println!("Sorry, you are not on the list.")
    // }
    //println!("hello, {}", name);
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    //greetings: String,
    //the age is a constraint check on alcohol
    age: i8
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {note: String},
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
           // greetings: greetings.to_string(),
           age,
        }
    }

    fn greet_visitor(&self) {
      match &self.action {
        VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
        VisitorAction::AcceptWithNote {note} => {
            println!("Welcome to the treehouse, {}", self.name);
            println!("{}", note);

            if self.age < 21 {
                println!("Do not serve alcohol to {}", self.name)
            }
        }

        VisitorAction::Probation => println!("{} is now a probationary member",  self.name),
        VisitorAction::Refuse => println!("Do not allow {} in!", self.name)
      }
    }
}

