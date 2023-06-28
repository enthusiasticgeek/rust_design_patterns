use std::rc::Rc;
use std::cell::RefCell;

struct RAV4 {
    parts: Vec<String>,
}

impl RAV4 {
    fn list_parts(&self) {
        print!("Product parts: ");
        for (i, part) in self.parts.iter().enumerate() {
            if i == self.parts.len() - 1 {
                print!("{}", part);
            } else {
                print!("{}, ", part);
            }
        }
        println!("\n");
    }
}

trait Builder {
    fn base_model(&self);
    fn blind_spot_monitoring(&self);
    fn lane_deviation_alert(&self);
    fn get_product(&self) -> Rc<RefCell<RAV4>>;
}

struct ConcreteBuilder1 {
    product: Rc<RefCell<RAV4>>,
}

impl Builder for ConcreteBuilder1 {
    fn base_model(&self) {
        self.product.borrow_mut().parts.push("RAV4 BASE MODEL".to_string());
    }

    fn blind_spot_monitoring(&self) {
        self.product.borrow_mut().parts.push("Blind Spot Monitoring".to_string());
    }

    fn lane_deviation_alert(&self) {
        self.product.borrow_mut().parts.push("Lane Deviation Alert".to_string());
    }

    fn get_product(&self) -> Rc<RefCell<RAV4>> {
        self.product.clone()
    }
}

impl ConcreteBuilder1 {
    fn new() -> Self {
        Self {
            product: Rc::new(RefCell::new(RAV4 { parts: Vec::new() })),
        }
    }
}

struct Director {
    builder: Option<Rc<dyn Builder>>,
}

impl Director {
    fn set_builder(&mut self, builder: Rc<dyn Builder>) {
        self.builder = Some(builder);
    }

    fn build_minimal_viable_product(&self) {
        self.builder.as_ref().unwrap().base_model();
    }

    fn build_full_featured_product(&self) {
        self.builder.as_ref().unwrap().base_model();
        self.builder.as_ref().unwrap().blind_spot_monitoring();
        self.builder.as_ref().unwrap().lane_deviation_alert();
    }
}

fn client_code(director: &mut Director) {
    let builder: Rc<dyn Builder> = Rc::new(ConcreteBuilder1::new());
    director.set_builder(builder.clone());

    println!("Standard basic product:");
    director.build_minimal_viable_product();

    let product = builder.get_product();
    product.borrow().list_parts();

    println!("Standard full featured product:");
    director.build_full_featured_product();

    let product = builder.get_product();
    product.borrow().list_parts();

    println!("Custom product:");
    builder.base_model();
    builder.lane_deviation_alert();

    let product = builder.get_product();
    product.borrow().list_parts();
}

fn main() {
    let mut director = Director { builder: None };
    client_code(&mut director);
}
