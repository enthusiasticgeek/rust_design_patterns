use std::fmt;

trait WritingInstrument: fmt::Debug {
    fn operation(&self) -> String;
}

#[derive(Debug)]
struct ConcretePen;

impl WritingInstrument for ConcretePen {
    fn operation(&self) -> String {
        String::from("{Pen}")
    }
}

#[derive(Debug)]
struct ConcretePencil;

impl WritingInstrument for ConcretePencil {
    fn operation(&self) -> String {
        String::from("{Pencil}")
    }
}

#[derive(Debug)]
struct ConcreteStylus;

impl WritingInstrument for ConcreteStylus {
    fn operation(&self) -> String {
        String::from("{Stylus}")
    }
}

trait WritingInstrumentCreator {
    fn factory_method(&self) -> Box<dyn WritingInstrument>;
    fn scribble(&self) -> String {
        let product = self.factory_method();
        format!("WritingInstrumentCreator: Scribbling with {:?}", product)
    }
}

struct ConcreteWritingInstrumentCreator1;

impl WritingInstrumentCreator for ConcreteWritingInstrumentCreator1 {
    fn factory_method(&self) -> Box<dyn WritingInstrument> {
        Box::new(ConcretePen)
    }
}

struct ConcreteWritingInstrumentCreator2;

impl WritingInstrumentCreator for ConcreteWritingInstrumentCreator2 {
    fn factory_method(&self) -> Box<dyn WritingInstrument> {
        Box::new(ConcretePencil)
    }
}

struct ConcreteWritingInstrumentCreator3;

impl WritingInstrumentCreator for ConcreteWritingInstrumentCreator3 {
    fn factory_method(&self) -> Box<dyn WritingInstrument> {
        Box::new(ConcreteStylus)
    }
}

fn writing_client_code(creator: Box<dyn WritingInstrumentCreator>) {
    println!("WritingClient: I'm oblivious to the creator's class, but it still works.");
    println!("{}", creator.scribble());
}

fn main() {
    println!("App: Launched with the ConcreteWritingInstrumentCreator1.");
    let creator1: Box<dyn WritingInstrumentCreator> = Box::new(ConcreteWritingInstrumentCreator1);
    writing_client_code(creator1);
    println!();

    println!("App: Launched with the ConcreteWritingInstrumentCreator2.");
    let creator2: Box<dyn WritingInstrumentCreator> = Box::new(ConcreteWritingInstrumentCreator2);
    writing_client_code(creator2);
    println!();

    println!("App: Launched with the ConcreteWritingInstrumentCreator3.");
    let creator3: Box<dyn WritingInstrumentCreator> = Box::new(ConcreteWritingInstrumentCreator3);
    writing_client_code(creator3);
}
