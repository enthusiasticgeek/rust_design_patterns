use std::rc::Rc;

trait Command {
    fn execute(&self);
}

struct SimpleCommand {
    payload: String,
}

impl SimpleCommand {
    fn new(payload: String) -> Self {
        SimpleCommand { payload }
    }
}

impl Command for SimpleCommand {
    fn execute(&self) {
        println!("SimpleCommand: See, I can do simple things like printing ({})", self.payload);
    }
}

struct Receiver;

impl Receiver {
    fn do_something(&self, a: &str) {
        println!("Receiver: Working on ({}.)", a);
    }

    fn do_something_else(&self, b: &str) {
        println!("Receiver: Also working on ({}.)", b);
    }
}

struct ComplexCommand {
    receiver: Rc<Receiver>,
    a: String,
    b: String,
}

impl ComplexCommand {
    fn new(receiver: Rc<Receiver>, a: String, b: String) -> Self {
        ComplexCommand { receiver, a, b }
    }
}

impl Command for ComplexCommand {
    fn execute(&self) {
        println!("ComplexCommand: Complex stuff should be done by a receiver object.");
        self.receiver.do_something(&self.a);
        self.receiver.do_something_else(&self.b);
    }
}

struct Invoker {
    on_start: Option<Rc<dyn Command>>,
    on_finish: Option<Rc<dyn Command>>,
}

impl Invoker {
    fn set_on_start(&mut self, command: Rc<dyn Command>) {
        self.on_start = Some(command);
    }

    fn set_on_finish(&mut self, command: Rc<dyn Command>) {
        self.on_finish = Some(command);
    }

    fn do_something_important(&self) {
        println!("Invoker: Does anybody want something done before I begin?");
        if let Some(command) = &self.on_start {
            command.execute();
        }
        println!("Invoker: ...doing something really important...");
        println!("Invoker: Does anybody want something done after I finish?");
        if let Some(command) = &self.on_finish {
            command.execute();
        }
    }
}

fn main() {
    let mut invoker = Invoker {
        on_start: None,
        on_finish: None,
    };

    invoker.set_on_start(Rc::new(SimpleCommand::new("Say Hi!".to_string())));
    let receiver = Rc::new(Receiver);
    invoker.set_on_finish(Rc::new(ComplexCommand::new(
        Rc::clone(&receiver),
        "Send email".to_string(),
        "Save report".to_string(),
    )));

    invoker.do_something_important();
}
