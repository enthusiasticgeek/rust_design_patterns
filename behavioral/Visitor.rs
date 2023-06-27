use std::rc::Rc;

trait Visitor {
    fn visit_concrete_component_a(&self, element: Rc<ConcreteComponentA>);
    fn visit_concrete_component_b(&self, element: Rc<ConcreteComponentB>);
}

trait Component {
    fn accept(&self, visitor: Rc<dyn Visitor>);
}
#[derive(Clone)]
struct ConcreteComponentA {}

impl Component for ConcreteComponentA {
    fn accept(&self, visitor: Rc<dyn Visitor>) {
        visitor.visit_concrete_component_a(Rc::new(self.clone()));
    }
}

impl ConcreteComponentA {
    fn exclusive_method_of_concrete_component_a(&self) -> String {
        // Placeholder method
        String::from("A")
    }
}
#[derive(Clone)]
struct ConcreteComponentB {}

impl Component for ConcreteComponentB {
    fn accept(&self, visitor: Rc<dyn Visitor>) {
        visitor.visit_concrete_component_b(Rc::new(self.clone()));
    }
}

impl ConcreteComponentB {
    fn special_method_of_concrete_component_b(&self) -> String {
        // Placeholder method
        String::from("B")
    }
}
#[derive(Clone)]
struct ConcreteVisitor1 {}

impl Visitor for ConcreteVisitor1 {
    fn visit_concrete_component_a(&self, element: Rc<ConcreteComponentA>) {
        println!(
            "{} + ConcreteVisitor1",
            element.exclusive_method_of_concrete_component_a()
        );
    }

    fn visit_concrete_component_b(&self, element: Rc<ConcreteComponentB>) {
        println!(
            "{} + ConcreteVisitor1",
            element.special_method_of_concrete_component_b()
        );
    }
}

#[derive(Clone)]
struct ConcreteVisitor2 {}

impl Visitor for ConcreteVisitor2 {
    fn visit_concrete_component_a(&self, element: Rc<ConcreteComponentA>) {
        println!(
            "{} + ConcreteVisitor2",
            element.exclusive_method_of_concrete_component_a()
        );
    }

    fn visit_concrete_component_b(&self, element: Rc<ConcreteComponentB>) {
        println!(
            "{} + ConcreteVisitor2",
            element.special_method_of_concrete_component_b()
        );
    }
}

fn client_code(components: Vec<Rc<dyn Component>>, visitor: Rc<dyn Visitor>) {
    for component in components {
        component.accept(visitor.clone());
    }
}

fn main() {
    let components: Vec<Rc<dyn Component>> = vec![
        Rc::new(ConcreteComponentA {}),
        Rc::new(ConcreteComponentB {}),
    ];

    let visitor1: Rc<dyn Visitor> = Rc::new(ConcreteVisitor1 {});
    println!("The client code works with all visitors via the base Visitor interface:");
    client_code(components.clone(), visitor1.clone());
    println!();

    let visitor2: Rc<dyn Visitor> = Rc::new(ConcreteVisitor2 {});
    println!("It allows the same client code to work with different types of visitors:");
    client_code(components, visitor2);
}
