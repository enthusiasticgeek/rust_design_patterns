use std::rc::Rc;

/**
 * Iterator Design Pattern
 *
 * Intent: Lets you traverse elements of a collection without exposing its
 * underlying representation (list, stack, tree, etc.).
 */

struct Iterator<T> {
    data: Rc<dyn Iterable<T>>,
    index: usize,
}

impl<T> Iterator<T> {
    fn new(data: Rc<dyn Iterable<T>>) -> Self {
        Self { data, index: 0 }
    }

    fn first(&mut self) {
        self.index = 0;
    }

    fn next(&mut self) {
        self.index += 1;
    }

    fn is_done(&self) -> bool {
        self.index >= self.data.size()
    }

    fn current(&self) -> &T {
        self.data.get(self.index)
    }
}

trait Iterable<T> {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> &T;
}

struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> {
    fn add(&mut self, item: T) {
        self.data.push(item);
    }
}

impl<T> Iterable<T> for Container<T> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> &T {
        &self.data[index]
    }
}

struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn client_code() {
    println!("________________Iterator with int______________________________________");
    let mut cont: Container<i32> = Container { data: Vec::new() };

    for i in 0..10 {
        cont.add(i);
    }

    let mut it = Iterator::new(Rc::new(cont));
    it.first();
    while !it.is_done() {
        println!("{}", it.current());
        it.next();
    }

    let mut cont2: Container<Data> = Container { data: Vec::new() };
    let a = Data::new(100);
    let b = Data::new(1000);
    let c = Data::new(10000);
    cont2.add(a);
    cont2.add(b);
    cont2.add(c);

    println!("________________Iterator with custom Class______________________________");
    let mut it2 = Iterator::new(Rc::new(cont2));
    it2.first();
    while !it2.is_done() {
        println!("{}", it2.current().value());
        it2.next();
    }
}

fn main() {
    client_code();
}
