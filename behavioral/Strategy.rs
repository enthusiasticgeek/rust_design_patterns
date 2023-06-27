use std::rc::Rc;

/// The Strategy trait declares operations common to all supported versions
/// of some algorithm.
///
/// The Context uses this trait to call the algorithm defined by Concrete
/// Strategies.
trait Strategy {
    fn do_algorithm(&self, data: &[String]) -> String;
}

/// The Context defines the interface of interest to clients.
struct Context {
    strategy: Option<Rc<dyn Strategy>>,
}

impl Context {
    /// Usually, the Context accepts a strategy through the constructor, but also
    /// provides a setter to change it at runtime.
    fn new(strategy: Option<Rc<dyn Strategy>>) -> Self {
        Context { strategy }
    }

    /// Usually, the Context allows replacing a Strategy object at runtime.
    fn set_strategy(&mut self, strategy: Option<Rc<dyn Strategy>>) {
        self.strategy = strategy;
    }

    /// The Context delegates some work to the Strategy object instead of
    /// implementing multiple versions of the algorithm on its own.
    fn do_some_business_logic(&self) {
        // ...
        println!("Context: Sorting data using the strategy (not sure how it'll do it)");
        let result = match &self.strategy {
            Some(strategy) => strategy.do_algorithm(&["a", "e", "c", "b", "d"].iter().map(|s| s.to_string()).collect::<Vec<String>>()),
            None => String::new(),
        };
        println!("{}", result);
        // ...
    }
}

/// Concrete Strategies implement the algorithm while following the base Strategy
/// trait. The trait makes them interchangeable in the Context.
struct ConcreteStrategyA;

impl Strategy for ConcreteStrategyA {
    fn do_algorithm(&self, data: &[String]) -> String {
        let mut result = String::new();
        for letter in data {
            result.push_str(letter);
        }
        let mut chars: Vec<char> = result.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    }
}

struct ConcreteStrategyB;

impl Strategy for ConcreteStrategyB {
    fn do_algorithm(&self, data: &[String]) -> String {
        let mut result = String::new();
        for letter in data {
            result.push_str(letter);
        }
        let mut chars: Vec<char> = result.chars().collect();
        let len = chars.len();
        for i in 0..(len / 2) {
            chars.swap(i, len - i - 1);
        }
        chars.into_iter().collect()
    }
}

/// The client code picks a concrete strategy and passes it to the context. The
/// client should be aware of the differences between strategies in order to make
/// the right choice.
fn client_code() {
    let mut context = Context::new(Some(Rc::new(ConcreteStrategyA)));
    println!("Client: Strategy is set to normal sorting.");
    context.do_some_business_logic();
    println!();
    println!("Client: Strategy is set to reverse sorting.");
    context.set_strategy(Some(Rc::new(ConcreteStrategyB)));
    context.do_some_business_logic();
}

fn main() {
    client_code();
}
