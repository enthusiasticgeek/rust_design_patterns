use std::rc::Rc;
use std::cell::RefCell;

trait Handler {
    fn set_next(&mut self, handler: Rc<RefCell<dyn Handler>>) -> Rc<RefCell<dyn Handler>>;
    fn handle(&self, request: &str) -> String;
}

struct AbstractHandler {
    next_handler: Option<Rc<RefCell<dyn Handler>>>,
}

impl AbstractHandler {
    fn _new() -> Self {
        Self {
            next_handler: None,
        }
    }
}

impl Handler for AbstractHandler {
    fn set_next(&mut self, handler: Rc<RefCell<dyn Handler>>) -> Rc<RefCell<dyn Handler>> {
        self.next_handler = Some(handler.clone());
        handler
    }

    fn handle(&self, request: &str) -> String {
        if let Some(ref next) = self.next_handler {
            next.borrow().handle(request)
        } else {
            String::new()
        }
    }
}

struct MonkeyHandler;

impl Handler for MonkeyHandler {
    fn set_next(&mut self, handler: Rc<RefCell<dyn Handler>>) -> Rc<RefCell<dyn Handler>> {
        handler
    }

    fn handle(&self, request: &str) -> String {
        if request == "Banana" {
            format!("Monkey: I'll eat the {}.\n", request)
        } else {
            String::new()
        }
    }
}

struct SquirrelHandler {
    next_handler: Option<Rc<RefCell<dyn Handler>>>,
}

impl SquirrelHandler {
    fn new() -> Self {
        Self {
            next_handler: None,
        }
    }
}

impl Handler for SquirrelHandler {
    fn set_next(&mut self, handler: Rc<RefCell<dyn Handler>>) -> Rc<RefCell<dyn Handler>> {
        self.next_handler = Some(handler.clone());
        handler
    }

    fn handle(&self, request: &str) -> String {
        if let Some(ref next) = self.next_handler {
            next.borrow().handle(request)
        } else {
            String::new()
        }
    }
}

struct DogHandler;

impl Handler for DogHandler {
    fn set_next(&mut self, handler: Rc<RefCell<dyn Handler>>) -> Rc<RefCell<dyn Handler>> {
        handler
    }

    fn handle(&self, request: &str) -> String {
        if request == "MeatBall" {
            format!("Dog: I'll eat the {}.\n", request)
        } else {
            String::new()
        }
    }
}

fn client_code(handler: Rc<RefCell<dyn Handler>>) {
    let food = vec!["Nut", "Banana", "Cup of coffee"];
    for f in food {
        println!("Client: Who wants a {}?", f);
        let result = handler.borrow().handle(f);
        if !result.is_empty() {
            println!("  {}", result);
        } else {
            println!("  {} was left untouched.", f);
        }
    }
}

fn main() {
    let monkey = Rc::new(RefCell::new(MonkeyHandler));
    let squirrel = Rc::new(RefCell::new(SquirrelHandler::new()));
    let dog = Rc::new(RefCell::new(DogHandler));

    let handler = monkey.clone();
    monkey.borrow_mut().set_next(squirrel.clone()).borrow_mut().set_next(dog.clone());

    println!("Chain: Monkey > Squirrel > Dog\n");
    client_code(handler.clone());
    println!();
    println!("Subchain: Squirrel > Dog\n");
    client_code(squirrel.clone());
}
