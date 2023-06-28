use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
struct SharedState {
    brand: String,
    model: String,
    color: String,
}

impl SharedState {
    fn new(brand: &str, model: &str, color: &str) -> Self {
        SharedState {
            brand: brand.to_string(),
            model: model.to_string(),
            color: color.to_string(),
        }
    }
}

impl fmt::Display for SharedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {}, {}, {} ]", self.brand, self.model, self.color)
    }
}

struct UniqueState {
    owner: String,
    plates: String,
}

impl UniqueState {
    fn new(owner: &str, plates: &str) -> Self {
        UniqueState {
            owner: owner.to_string(),
            plates: plates.to_string(),
        }
    }
}

impl fmt::Display for UniqueState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {}, {} ]", self.owner, self.plates)
    }
}

struct Flyweight {
    shared_state: SharedState,
}

impl Flyweight {
    fn new(shared_state: SharedState) -> Self {
        Flyweight { shared_state }
    }

    fn operation(&self, unique_state: UniqueState) {
        println!(
            "Flyweight: Displaying shared ({}) and unique ({}) state.",
            self.shared_state, unique_state
        );
    }
}

struct FlyweightFactory {
    flyweights: HashMap<String, Flyweight>,
}

impl FlyweightFactory {
    fn new(shared_states: Vec<SharedState>) -> Self {
        let mut flyweights = HashMap::new();
        for shared_state in shared_states {
            let key = FlyweightFactory::get_key(&shared_state);
            flyweights.insert(key, Flyweight::new(shared_state));
        }
        FlyweightFactory { flyweights }
    }

    fn get_key(shared_state: &SharedState) -> String {
        format!("{}_{}_{}", shared_state.brand, shared_state.model, shared_state.color)
    }

    fn get_flyweight(&mut self, shared_state: SharedState) -> &Flyweight {
        let key = FlyweightFactory::get_key(&shared_state);
        if !self.flyweights.contains_key(&key) {
            println!("FlyweightFactory: Can't find a flyweight, creating a new one.");
            self.flyweights.insert(key.clone(), Flyweight::new(shared_state));
        } else {
            println!("FlyweightFactory: Reusing existing flyweight.");
        }
        self.flyweights.get(&key).unwrap()
    }

    fn list_flyweights(&self) {
        let count = self.flyweights.len();
        println!("FlyweightFactory: I have {} flyweights:", count);
        for key in self.flyweights.keys() {
            println!("{}", key);
        }
    }
}

fn add_car_to_police_database(
    factory: &mut FlyweightFactory,
    plates: &str,
    owner: &str,
    brand: &str,
    model: &str,
    color: &str,
) {
    println!("Client: Adding a car to the database.");
    let shared_state = SharedState::new(brand, model, color);
    let flyweight = factory.get_flyweight(shared_state);
    let unique_state = UniqueState::new(owner, plates);
    flyweight.operation(unique_state);
}

fn main() {
    let mut factory = FlyweightFactory::new(vec![
        SharedState::new("Chevrolet", "Camaro2018", "pink"),
        SharedState::new("Mercedes Benz", "C300", "black"),
        SharedState::new("Mercedes Benz", "C500", "red"),
        SharedState::new("BMW", "M5", "red"),
        SharedState::new("BMW", "X6", "white"),
    ]);

    factory.list_flyweights();

    add_car_to_police_database(
        &mut factory,
        "CL234IR",
        "James Doe",
        "BMW",
        "M5",
        "red",
    );

    add_car_to_police_database(
        &mut factory,
        "CL234IR",
        "James Doe",
        "BMW",
        "X1",
        "red",
    );

    factory.list_flyweights();
}
