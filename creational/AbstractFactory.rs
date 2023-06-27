use std::rc::Rc;

trait AbstractCompact {
    fn useful_function_a(&self) -> String;
}

trait AbstractMidSize {
    fn useful_function_b(&self) -> String;
    fn another_useful_function_b(&self, collaborator: Rc<dyn AbstractCompact>) -> String;
}

struct ConcreteCorolla;

impl AbstractCompact for ConcreteCorolla {
    fn useful_function_a(&self) -> String {
        "The result of the product: Toyota Corolla.".to_string()
    }
}

struct ConcreteCivic;

impl AbstractCompact for ConcreteCivic {
    fn useful_function_a(&self) -> String {
        "The result of the product: Honda Civic.".to_string()
    }
}

struct ConcreteCamry;

impl AbstractMidSize for ConcreteCamry {
    fn useful_function_b(&self) -> String {
        "The result of the product: Toyota Camry.".to_string()
    }

    fn another_useful_function_b(&self, collaborator: Rc<dyn AbstractCompact>) -> String {
        let result = collaborator.useful_function_a();
        format!("The result of the B1 collaborating with ( {} )", result)
    }
}

struct ConcreteAccord;

impl AbstractMidSize for ConcreteAccord {
    fn useful_function_b(&self) -> String {
        "The result of the product: Honda Accord.".to_string()
    }

    fn another_useful_function_b(&self, collaborator: Rc<dyn AbstractCompact>) -> String {
        let result = collaborator.useful_function_a();
        format!("The result of the B2 collaborating with ( {} )", result)
    }
}

trait AbstractFactory {
    fn create_compact(&self) -> Rc<dyn AbstractCompact>;
    fn create_midsize(&self) -> Rc<dyn AbstractMidSize>;
}

struct ConcreteToyota;

impl AbstractFactory for ConcreteToyota {
    fn create_compact(&self) -> Rc<dyn AbstractCompact> {
        Rc::new(ConcreteCorolla)
    }

    fn create_midsize(&self) -> Rc<dyn AbstractMidSize> {
        Rc::new(ConcreteCamry)
    }
}

struct ConcreteHonda;

impl AbstractFactory for ConcreteHonda {
    fn create_compact(&self) -> Rc<dyn AbstractCompact> {
        Rc::new(ConcreteCivic)
    }

    fn create_midsize(&self) -> Rc<dyn AbstractMidSize> {
        Rc::new(ConcreteAccord)
    }
}

fn client_code(factory: Rc<dyn AbstractFactory>) {
    let compact = factory.create_compact();
    let midsize = factory.create_midsize();

    println!("{}", midsize.useful_function_b());
    println!("{}", midsize.another_useful_function_b(Rc::clone(&compact)));
}

fn main() {
    println!("Client: Testing client code with the first factory type:");
    let toyota: Rc<dyn AbstractFactory> = Rc::new(ConcreteToyota);
    client_code(Rc::clone(&toyota));

    println!("Client: Testing the same client code with the second factory type:");
    let honda: Rc<dyn AbstractFactory> = Rc::new(ConcreteHonda);
    client_code(Rc::clone(&honda));
}
