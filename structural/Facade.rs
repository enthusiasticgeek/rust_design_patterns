struct Subsystem1;

impl Subsystem1 {
    fn operation1(&self) -> String {
        String::from("Subsystem1: Ready!\n")
    }
    
    // ...
    
    fn operation_n(&self) -> String {
        String::from("Subsystem1: Go!\n")
    }
}

struct Subsystem2;

impl Subsystem2 {
    fn operation1(&self) -> String {
        String::from("Subsystem2: Get ready!\n")
    }
    
    // ...
    
    fn operation_z(&self) -> String {
        String::from("Subsystem2: Fire!\n")
    }
}

struct Facade {
    subsystem1: Subsystem1,
    subsystem2: Subsystem2,
}

impl Facade {
    fn new(subsystem1: Subsystem1, subsystem2: Subsystem2) -> Self {
        Facade {
            subsystem1,
            subsystem2,
        }
    }
    
    fn operation(&self) -> String {
        let mut result = String::from("Facade initializes subsystems:\n");
        result += &self.subsystem1.operation1();
        result += &self.subsystem2.operation1();
        result += "Facade orders subsystems to perform the action:\n";
        result += &self.subsystem1.operation_n();
        result += &self.subsystem2.operation_z();
        result
    }
}

fn client_code(facade: &Facade) {
    // ...
    println!("{}", facade.operation());
    // ...
}

fn main() {
    let subsystem1 = Subsystem1;
    let subsystem2 = Subsystem2;
    let facade = Facade::new(subsystem1, subsystem2);
    client_code(&facade);
}
