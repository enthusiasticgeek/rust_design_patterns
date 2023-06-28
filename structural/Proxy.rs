use std::rc::Rc;

trait Subject {
    fn request(&self);
}

struct RealSubject;

impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: Handling request.");
    }
}

struct Proxy {
    real_subject: Rc<dyn Subject>,
}

impl Proxy {
    fn new(real_subject: Rc<dyn Subject>) -> Self {
        Proxy { real_subject }
    }

    fn check_access(&self) -> bool {
        // Some real checks should go here.
        println!("Proxy: Checking access prior to firing a real request.");
        true
    }

    fn log_access(&self) {
        println!("Proxy: Logging the time of request.");
    }
}

impl Subject for Proxy {
    fn request(&self) {
        if self.check_access() {
            self.real_subject.request();
            self.log_access();
        }
    }
}

fn client_code(subject: Rc<dyn Subject>) {
    subject.request();
}

fn main() {
    println!("Client: Executing the client code with a real subject:");
    let real_subject = Rc::new(RealSubject);
    client_code(real_subject.clone());

    println!("\nClient: Executing the same client code with a proxy:");
    let proxy = Rc::new(Proxy::new(real_subject));
    client_code(proxy);
}
