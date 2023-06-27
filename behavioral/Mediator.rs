use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)] // Implement Debug for Mediator
struct Mediator {
    component1: Weak<RefCell<Component1>>,
    component2: Weak<RefCell<Component2>>,
}

impl Mediator {
    fn set_components(&mut self, c1: Rc<RefCell<Component1>>, c2: Rc<RefCell<Component2>>) {
        self.component1 = Rc::downgrade(&c1);
        self.component2 = Rc::downgrade(&c2);
    }

    fn notify<T: BaseComponent + 'static>(&self, event: &str) {
        if event == "A" {
            println!("Mediator reacts on A and triggers the following operations:");
            if let Some(component2) = self.component2.upgrade() {
                component2.borrow().do_c();
            }
        }
        if event == "D" {
            println!("Mediator reacts on D and triggers the following operations:");
            if let Some(component1) = self.component1.upgrade() {
                component1.borrow().do_b();
            }
            if let Some(component2) = self.component2.upgrade() {
                component2.borrow().do_c();
            }
        }
    }
}

trait BaseComponent: std::fmt::Debug {
    fn shared_from_this(&self) -> Rc<RefCell<Self>>;
}

#[derive(Clone)]
#[derive(Debug)] // Implement Debug for Component1
struct Component1 {
    mediator: Option<Rc<RefCell<Mediator>>>,
}

impl BaseComponent for Component1 {
    fn shared_from_this(&self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self.clone()))
    }
}

impl Component1 {
    fn do_a(&self) {
        println!("Component 1 does A.");
        if let Some(mediator) = self.mediator.as_ref() {
            mediator.borrow().notify::<Component1>("A");
        }
    }

    fn do_b(&self) {
        println!("Component 1 does B.");
        if let Some(mediator) = self.mediator.as_ref() {
            mediator.borrow().notify::<Component1>("B");
        }
    }
}

#[derive(Clone)]
#[derive(Debug)] // Implement Debug for Component2
struct Component2 {
    mediator: Option<Rc<RefCell<Mediator>>>,
}

impl BaseComponent for Component2 {
    fn shared_from_this(&self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self.clone()))
    }
}

impl Component2 {
    fn do_c(&self) {
        println!("Component 2 does C.");
        if let Some(mediator) = self.mediator.as_ref() {
            mediator.borrow().notify::<Component2>("C");
        }
    }

    fn do_d(&self) {
        println!("Component 2 does D.");
        if let Some(mediator) = self.mediator.as_ref() {
            mediator.borrow().notify::<Component2>("D");
        }
    }
}

fn client_code() {
    let c1: Rc<RefCell<Component1>> = Rc::new(RefCell::new(Component1 { mediator: None }));
    let c2: Rc<RefCell<Component2>> = Rc::new(RefCell::new(Component2 { mediator: None }));
    let mediator: Rc<RefCell<Mediator>> = Rc::new(RefCell::new(Mediator {
        component1: Weak::default(),
        component2: Weak::default(),
    }));

    mediator.borrow_mut().set_components(c1.clone(), c2.clone());

    c1.borrow_mut().mediator = Some(mediator.clone());
    c2.borrow_mut().mediator = Some(mediator.clone());

    println!("Client triggers operation A.");
    c1.borrow().do_a();
    println!();
    println!("Client triggers operation D.");
    c2.borrow().do_d();
}

fn main() {
    client_code();
}
