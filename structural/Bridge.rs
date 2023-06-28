use std::rc::Rc;

trait Implementation {
    fn operation_implementation(&self) -> String;
}

struct ConcreteImplementationA;

impl Implementation for ConcreteImplementationA {
    fn operation_implementation(&self) -> String {
        String::from("ConcreteImplementationA: Here's the result on the platform A.\n")
    }
}

struct ConcreteImplementationB;

impl Implementation for ConcreteImplementationB {
    fn operation_implementation(&self) -> String {
        String::from("ConcreteImplementationB: Here's the result on the platform B.\n")
    }
}

struct Abstraction {
    implementation: Rc<dyn Implementation>,
}

impl Abstraction {
    fn new(implementation: Rc<dyn Implementation>) -> Self {
        Abstraction { implementation }
    }

    fn operation(&self) -> String {
        format!(
            "Abstraction: Base operation with:\n{}",
            self.implementation.operation_implementation()
        )
    }
}

struct ExtendedAbstraction {
    implementation: Rc<dyn Implementation>,
}

impl ExtendedAbstraction {
    fn operation(&self) -> String {
        format!(
            "ExtendedAbstraction: Extended operation with:\n{}",
            self.implementation.operation_implementation()
        )
    }
}

fn client_code(abstraction: &dyn AbstractionTrait) {
    println!("{}", abstraction.operation());
}

trait AbstractionTrait {
    fn operation(&self) -> String;
}

impl AbstractionTrait for Abstraction {
    fn operation(&self) -> String {
        self.operation()
    }
}

impl AbstractionTrait for ExtendedAbstraction {
    fn operation(&self) -> String {
        self.operation()
    }
}

fn main() {
    let implementation_a: Rc<dyn Implementation> = Rc::new(ConcreteImplementationA {});
    let abstraction_a: Rc<dyn AbstractionTrait> = Rc::new(Abstraction::new(implementation_a));
    client_code(&*abstraction_a);

    println!();

    let implementation_b: Rc<dyn Implementation> = Rc::new(ConcreteImplementationB {});
    let abstraction_b: Rc<dyn AbstractionTrait> = Rc::new(Abstraction::new(implementation_b));
    client_code(&*abstraction_b);
}
