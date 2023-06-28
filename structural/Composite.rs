use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

trait Component: Any {
    fn operation(&self) -> String;
}

struct Leaf;

impl Component for Leaf {
    fn operation(&self) -> String {
        String::from("Leaf")
    }
}

struct Composite {
    children: RefCell<Vec<Rc<dyn Component>>>,
}

impl Composite {
    fn new() -> Self {
        Composite {
            children: RefCell::new(vec![]),
        }
    }

    fn add(&self, component: Rc<dyn Component>) {
        self.children.borrow_mut().push(component);
    }

    fn remove(&self, component: Rc<dyn Component>) {
        let mut children = self.children.borrow_mut();
        if let Some(index) = children.iter().position(|c| Rc::ptr_eq(c, &component)) {
            children.remove(index);
        }
    }
}

impl Component for Composite {
    fn operation(&self) -> String {
        let result = self
            .children
            .borrow()
            .iter()
            .map(|c| c.operation())
            .collect::<Vec<String>>()
            .join("+");
        format!("Branch({})", result)
    }
}

fn client_code(component: Rc<dyn Component>) {
    println!("RESULT: {}", component.operation());
}

fn client_code2(component1: Rc<dyn Component>, component2: Rc<dyn Component>) {
    if let Some(composite) = component1.as_any().downcast_ref::<Composite>() {
        composite.add(component2);
    }
    println!("RESULT: {}", component1.operation());
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let simple = Rc::new(Leaf);
    println!("Client: I've got a simple component:");
    client_code(simple.clone());
    println!();

    let tree = Rc::new(Composite::new());
    let branch1 = Rc::new(Composite::new());
    let leaf_1: Rc<dyn Component> = Rc::new(Leaf);
    let leaf_2: Rc<dyn Component> = Rc::new(Leaf);
    let leaf_3: Rc<dyn Component> = Rc::new(Leaf);
    branch1.add(leaf_1.clone());
    branch1.add(leaf_2.clone());
    let branch2 = Rc::new(Composite::new());
    branch2.add(leaf_3.clone());
    tree.add(branch1.clone()); // Clone branch1
    tree.add(branch2);
    println!("Client: Now I've got a composite tree:");
    client_code(tree.clone());
    println!();

    // Removing leaf_2 from branch1
    branch1.remove(leaf_2.clone());
    println!("Client: After removing leaf_2 from branch1:");
    client_code(tree.clone());
    println!();

    println!("Client: I don't need to check the component classes even when managing the tree:");
    client_code2(tree.clone(), simple.clone());
}
