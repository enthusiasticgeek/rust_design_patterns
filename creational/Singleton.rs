use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Singleton {
    value: String,
}

impl Singleton {
    fn new(value: String) -> Self {
        Singleton { value }
    }

    fn some_business_logic(&self) {
        println!("Doing some business logic with value: {}", self.value);
    }

    fn value(&self) -> &str {
        &self.value
    }
}

struct SingletonManager {
    singleton: Arc<Mutex<Option<Singleton>>>,
}

impl SingletonManager {
    fn get_instance(value: String) -> Arc<Mutex<Option<Singleton>>> {
        lazy_static::lazy_static! {
            static ref SINGLETON_MANAGER: SingletonManager = SingletonManager {
                singleton: Arc::new(Mutex::new(None)),
            };
        }

        let mut instance = SINGLETON_MANAGER.singleton.lock().unwrap();
        if instance.is_none() {
            *instance = Some(Singleton::new(value));
        }

        Arc::clone(&SINGLETON_MANAGER.singleton)
    }
}

fn thread_foo() {
    // Following code emulates slow initialization.
    thread::sleep(Duration::from_millis(1000));
    let singleton = SingletonManager::get_instance("FOO".to_string());
    println!("{}", singleton.lock().unwrap().as_ref().unwrap().value());
    singleton.lock().unwrap().as_ref().unwrap().some_business_logic();
}

fn thread_bar() {
    // Following code emulates slow initialization.
    thread::sleep(Duration::from_millis(1000));
    let singleton = SingletonManager::get_instance("BAR".to_string());
    println!("{}", singleton.lock().unwrap().as_ref().unwrap().value());
    singleton.lock().unwrap().as_ref().unwrap().some_business_logic();
}

fn main() {
    println!(
        "If you see the same value, then singleton was reused (yay!)\n\
         If you see different values, then 2 singletons were created (booo!!)\n\n\
         RESULT:"
    );
    let t1 = thread::spawn(thread_bar);
    let t2 = thread::spawn(thread_foo);
    t1.join().unwrap();
    t2.join().unwrap();
}
