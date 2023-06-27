use std::rc::Rc;

trait AbstractClass {
    fn template_method(&self) {
        self.base_operation1();
        self.required_operations1();
        self.base_operation2();
        self.hook1();
        self.required_operation2();
        self.base_operation3();
        self.hook2();
    }

    fn base_operation1(&self) {
        println!("AbstractClass says: I am doing the bulk of the work");
    }

    fn base_operation2(&self) {
        println!("AbstractClass says: But I let subclasses override some operations");
    }

    fn base_operation3(&self) {
        println!("AbstractClass says: But I am doing the bulk of the work anyway");
    }

    fn required_operations1(&self);

    fn required_operation2(&self);

    fn hook1(&self) {}

    fn hook2(&self) {}
}

struct ConcreteClass1;

impl AbstractClass for ConcreteClass1 {
    fn required_operations1(&self) {
        println!("ConcreteClass1 says: Implemented Operation1");
    }

    fn required_operation2(&self) {
        println!("ConcreteClass1 says: Implemented Operation2");
    }
}

struct ConcreteClass2;

impl AbstractClass for ConcreteClass2 {
    fn required_operations1(&self) {
        println!("ConcreteClass2 says: Implemented Operation1");
    }

    fn required_operation2(&self) {
        println!("ConcreteClass2 says: Implemented Operation2");
    }

    fn hook1(&self) {
        println!("ConcreteClass2 says: Overridden Hook1");
    }
}

fn client_code(class_: Rc<dyn AbstractClass>) {
    class_.template_method();
}

fn main() {
    println!("Same client code can work with different subclasses:");
    let concrete_class1 = Rc::new(ConcreteClass1);
    client_code(concrete_class1);
    println!();

    println!("Same client code can work with different subclasses:");
    let concrete_class2 = Rc::new(ConcreteClass2);
    client_code(concrete_class2);
}
