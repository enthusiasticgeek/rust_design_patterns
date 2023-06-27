use std::cell::RefCell;
use std::rc::{Rc, Weak};

trait IObserver {
    fn update(&self, message_from_subject: &str);
}

trait ISubject {
    fn attach(&mut self, observer: Rc<dyn IObserver>);
    fn detach(&mut self, observer: Rc<dyn IObserver>);
    fn notify(&self);
}

struct Subject {
    observers: RefCell<Vec<Weak<dyn IObserver>>>,
    message: String,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: RefCell::new(Vec::new()),
            message: String::new(),
        }
    }

    fn how_many_observer(&self) {
        let count = self.observers.borrow().len();
        println!("There are {} observers in the list.", count);
    }

    fn create_message(&mut self, message: &str) {
        self.message = message.to_string();
        self.notify();
    }

    fn some_business_logic(&mut self) {
        self.message = "Changed message".to_string();
        self.notify();
        println!("I'm about to do something important.");
    }
}

impl ISubject for Subject {
    fn attach(&mut self, observer: Rc<dyn IObserver>) {
        self.observers.borrow_mut().push(Rc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Rc<dyn IObserver>) {
        let pos = self
            .observers
            .borrow()
            .iter()
            .position(|obs| {
                obs.upgrade()
                    .map_or(false, |ref_weak| Rc::ptr_eq(&ref_weak, &observer))
            })
            .unwrap();
        self.observers.borrow_mut().remove(pos);
        println!("Observer removed from the list.");
    }

    fn notify(&self) {
        let observers = self.observers.borrow();
        for observer in observers.iter() {
            if let Some(observer) = observer.upgrade() {
                observer.update(&self.message);
            }
        }
    }
}

struct Observer {
    subject: Option<Rc<RefCell<Subject>>>,
    number: usize,
}

impl Observer {
    fn new(subject: Rc<RefCell<Subject>>) -> Self {
        Observer {
            subject: Some(subject),
            number: 0,
        }
    }

    fn attach_to_subject(&mut self) {
        if let Some(subject) = self.subject.take() {
            subject.borrow_mut().attach(Rc::new(self.clone()));
        }
    }

    fn remove_me_from_the_list(&mut self) {
        if let Some(subject) = &self.subject {
            subject.borrow_mut().detach(Rc::new(self.clone()));
        }
        println!("Observer {} removed from the list.", self.number);
    }
}

impl IObserver for Observer {
    fn update(&self, message_from_subject: &str) {
        println!(
            "Observer {}: a new message is available --> {}",
            self.number, message_from_subject
        );
    }
}

impl Clone for Observer {
    fn clone(&self) -> Self {
        Observer {
            subject: self.subject.clone(),
            number: self.number + 1,
        }
    }
}

fn client_code() {
    let subject = Rc::new(RefCell::new(Subject::new()));

    subject.borrow_mut().create_message("Welcome! :D");
    let observer1 = Rc::new(RefCell::new(Observer::new(Rc::clone(&subject))));
    let observer2 = Rc::new(RefCell::new(Observer::new(Rc::clone(&subject))));
    let observer3 = Rc::new(RefCell::new(Observer::new(Rc::clone(&subject))));

    observer1.borrow_mut().attach_to_subject();
    observer2.borrow_mut().attach_to_subject();
    observer3.borrow_mut().attach_to_subject();

    observer1.borrow_mut().remove_me_from_the_list();

    subject.borrow_mut().create_message("Hello there!");

    observer2.borrow_mut().remove_me_from_the_list();

    subject.borrow_mut().some_business_logic();

    subject.borrow().how_many_observer();
}

fn main() {
    client_code();
}
