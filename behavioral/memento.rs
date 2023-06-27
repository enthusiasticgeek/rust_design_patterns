use std::time::{SystemTime, UNIX_EPOCH};
use std::thread::sleep;
use std::time::Duration;
use std::any::Any;

trait IMemento {
    fn get_name(&self) -> String;
    fn get_state(&self) -> String;
    fn get_date(&self) -> SystemTime;
    fn as_any(&self) -> &dyn Any;
}

struct ConcreteMemento {
    state: String,
    date: SystemTime,
}

impl ConcreteMemento {
    fn new(state: String) -> Self {
        Self {
            state,
            date: SystemTime::now(),
        }
    }
}

impl IMemento for ConcreteMemento {
    fn get_state(&self) -> String {
        self.state.clone()
    }

    fn get_name(&self) -> String {
        let epoch = self.date.duration_since(UNIX_EPOCH).unwrap();
        format!("{:?} / ({})...", epoch.as_secs(), &self.state[..9])
    }

    fn get_date(&self) -> SystemTime {
        self.date
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Originator {
    state: String,
}

impl Originator {
    fn new(state: String) -> Self {
        println!("Originator: My initial state is: {}", state);
        Self { state }
    }

    fn do_something(&mut self) {
        println!("Originator: I'm doing something important.");
        self.state = self.generate_random_string(30);
        println!("Originator: and my state has changed to: {}", self.state);
    }

    fn generate_random_string(&self, length: usize) -> String {
        let allowed_symbols = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut result = String::new();

        for _ in 0..length {
            let index = rand::random::<usize>() % allowed_symbols.len();
            result.push(allowed_symbols.chars().nth(index).unwrap());
            sleep(Duration::from_millis(12));
        }

        result
    }

    fn save(&self) -> Box<dyn IMemento> {
        Box::new(ConcreteMemento::new(self.state.clone()))
    }

    fn restore(&mut self, memento: &dyn IMemento) {
        if let Some(concrete_memento) = memento.as_any().downcast_ref::<ConcreteMemento>() {
            self.state = concrete_memento.get_state();
            println!("Originator: My state has changed to: {}", self.state);
        } else {
            panic!("Unknown memento class");
        }
    }
}

struct Caretaker<'a> {
    mementos: Vec<Box<dyn IMemento>>,
    originator: &'a mut Originator,
}

impl<'a> Caretaker<'a> {
    fn new(originator: &'a mut Originator) -> Self {
        Self {
            mementos: vec![],
            originator,
        }
    }

    fn backup(&mut self) {
        println!("\nCaretaker: Saving Originator's state...");
        self.mementos.push(self.originator.save());
    }

    fn undo(&mut self) {
        if let Some(memento) = self.mementos.pop() {
            println!("Caretaker: Restoring state to: {}", memento.get_name());
            self.originator.restore(&*memento);
        }
    }

    fn show_history(&self) {
        println!("Caretaker: Here's the list of mementos:");
        for memento in &self.mementos {
            println!("{}", memento.get_name());
        }
    }
}

fn main() {
    let mut originator = Originator::new("Super-duper-super-puper-super.".to_string());
    let mut caretaker = Caretaker::new(&mut originator);

    caretaker.backup();

    {
        let originator_ref = &mut caretaker.originator;
        originator_ref.do_something();
    }

    caretaker.backup();

    {
        let originator_ref = &mut caretaker.originator;
        originator_ref.do_something();
    }

    caretaker.backup();

    {
        let originator_ref = &mut caretaker.originator;
        originator_ref.do_something();
    }

    println!();
    caretaker.show_history();

    println!("\nClient: Now, let's rollback!\n");
    caretaker.undo();

    println!("\n\nClient: Once more!\n");
    caretaker.undo();

    println!();
}
