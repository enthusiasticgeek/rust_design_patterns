use std::rc::Rc;

trait Target {
    fn request(&self) -> String {
        String::from("Target: The default target's behavior.")
    }
}

struct Adaptee;

impl Adaptee {
    fn specific_request(&self) -> String {
        String::from(".eetpadA eht fo roivaheb laicepS")
    }
}

struct Adapter {
    adaptee: Rc<Adaptee>,
}

impl Adapter {
    fn new(adaptee: Rc<Adaptee>) -> Self {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        let to_reverse = self.adaptee.specific_request();
        let reversed: String = to_reverse.chars().rev().collect();
        format!("Adapter: (TRANSLATED) {}", reversed)
    }
}

fn client_code(target: &dyn Target) {
    println!("{}", target.request());
}

fn main() {
    println!("Client: I can work just fine with the Target objects:");
    let target: Rc<dyn Target> = Rc::new(Adapter::new(Rc::new(Adaptee {})));
    client_code(&*target);
    println!("\n");

    println!("Client: The Adaptee class has a weird interface. See, I don't understand it:");
    let adaptee = Adaptee {};
    println!("Adaptee: {}", adaptee.specific_request());
    println!("\n");

    println!("Client: But I can work with it via the Adapter:");
    let adapter = Adapter::new(Rc::new(adaptee));
    client_code(&adapter);
}
