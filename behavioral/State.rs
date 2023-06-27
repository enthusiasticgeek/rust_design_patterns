use std::any::type_name;
use std::cell::RefCell;
use std::rc::Rc;

trait State {
    fn handle1(&mut self, context: &mut Context);
    fn handle2(&mut self, context: &mut Context);
}

struct Context<'a> {
    state: Option<Rc<RefCell<dyn State + 'a>>>,
}

impl<'a> Context<'a> {
    fn new(state: Rc<RefCell<dyn State + 'a>>) -> Self {
        Self { state: Some(state) }
    }

    fn transition_to(&mut self, state: Rc<RefCell<dyn State + 'a>>) {
        println!("Context: Transition to {}", type_name::<dyn State>());
        self.state = Some(state);
    }

    fn request1(&mut self) {
        if let Some(state) = self.state.as_ref().map(Rc::clone) {
            state.borrow_mut().handle1(self);
        }
    }

    fn request2(&mut self) {
        if let Some(state) = self.state.as_ref().map(Rc::clone) {
            state.borrow_mut().handle2(self);
        }
    }
}

struct ConcreteStateA;

impl State for ConcreteStateA {
    fn handle1(&mut self, context: &mut Context) {
        println!("ConcreteStateA handles request1");
        println!("ConcreteStateA wants to change the state of the context");
        let state_b = Rc::new(RefCell::new(ConcreteStateB));
        context.transition_to(state_b);
    }

    fn handle2(&mut self, context: &mut Context) {
        println!("ConcreteStateA handles request2");
        println!("ConcreteStateA wants to change the state of the context");
        let state_b = Rc::new(RefCell::new(ConcreteStateB));
        context.transition_to(state_b);
    }
}

struct ConcreteStateB;

impl State for ConcreteStateB {
    fn handle1(&mut self, context: &mut Context) {
        println!("ConcreteStateB handles request1");
        let state_a = Rc::new(RefCell::new(ConcreteStateA));
        context.transition_to(state_a);
    }

    fn handle2(&mut self, context: &mut Context) {
        println!("ConcreteStateB handles request2");
        println!("ConcreteStateB wants to change the state of the context");
        let state_a = Rc::new(RefCell::new(ConcreteStateA));
        context.transition_to(state_a);
    }
}

fn main() {
    let state_a = Rc::new(RefCell::new(ConcreteStateA));
    let mut context1 = Context::new(state_a);
    context1.request1();
    context1.request2();
}
