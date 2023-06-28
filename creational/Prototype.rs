use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Type {
    Prototype1,
    Prototype2,
}

trait Prototype {
    fn method(&mut self, prototype_field: f32);
    fn clone_box(&self) -> Box<dyn Prototype>;
}

impl Clone for Box<dyn Prototype> {
    fn clone(&self) -> Box<dyn Prototype> {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
struct ConcretePrototype1 {
    prototype_name: String,
    concrete_prototype_field1: f32,
}

impl Prototype for ConcretePrototype1 {
    fn method(&mut self, prototype_field: f32) {
        self.concrete_prototype_field1 = prototype_field;
        println!(
            "Call Method from {} with field: {}",
            self.prototype_name, self.concrete_prototype_field1
        );
    }

    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
struct ConcretePrototype2 {
    prototype_name: String,
    concrete_prototype_field2: f32,
}

impl Prototype for ConcretePrototype2 {
    fn method(&mut self, prototype_field: f32) {
        self.concrete_prototype_field2 = prototype_field;
        println!(
            "Call Method from {} with field: {}",
            self.prototype_name, self.concrete_prototype_field2
        );
    }

    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
}

struct PrototypeFactory {
    prototypes: HashMap<Type, Box<dyn Prototype>>,
}

impl PrototypeFactory {
    fn new() -> Self {
        let mut prototypes = HashMap::new();
        prototypes.insert(
            Type::Prototype1,
            Box::new(ConcretePrototype1 {
                prototype_name: "PROTOTYPE_1".to_string(),
                concrete_prototype_field1: 50.0,
            }) as Box<dyn Prototype>,
        );
        prototypes.insert(
            Type::Prototype2,
            Box::new(ConcretePrototype2 {
                prototype_name: "PROTOTYPE_2".to_string(),
                concrete_prototype_field2: 60.0,
            }) as Box<dyn Prototype>,
        );

        PrototypeFactory { prototypes }
    }

    fn create_prototype(&self, type_: Type) -> Box<dyn Prototype> {
        self.prototypes[&type_].clone_box()
    }
}

fn client(prototype_factory: &PrototypeFactory) {
    println!("Let's create a Prototype 1");
    let mut prototype = prototype_factory.create_prototype(Type::Prototype1);
    prototype.method(90.0);
    println!();

    println!("Let's create a Prototype 2");
    prototype = prototype_factory.create_prototype(Type::Prototype2);
    prototype.method(10.0);
}

fn main() {
    let prototype_factory = PrototypeFactory::new();
    client(&prototype_factory);
}
