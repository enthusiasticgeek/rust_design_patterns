use std::rc::Rc;

trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        String::from("ConcreteComponent")
    }
}

struct Decorator {
    component: Rc<dyn Component>,
}

impl Component for Decorator {
    fn operation(&self) -> String {
        self.component.operation()
    }
}

struct ConcreteDecoratorA {
    component: Rc<dyn Component>,
}

impl Component for ConcreteDecoratorA {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorA({})", self.component.operation())
    }
}

struct ConcreteDecoratorB {
    component: Rc<dyn Component>,
}

impl Component for ConcreteDecoratorB {
    fn operation(&self) -> String {
        format!("ConcreteDecoratorB({})", self.component.operation())
    }
}

fn client_code(component: Rc<dyn Component>) {
    println!("RESULT: {}", component.operation());
}

fn main() {
    // Simple component
    let simple = Rc::new(ConcreteComponent);
    println!("Client: I've got a simple component:");
    client_code(simple.clone());
    println!();

    // Decorated components
    let decorator1 = Rc::new(ConcreteDecoratorA { component: simple.clone() });
    let decorator2 = Rc::new(ConcreteDecoratorB { component: decorator1.clone() });
    println!("Client: Now I've got a decorated component:");
    client_code(decorator2);
}
