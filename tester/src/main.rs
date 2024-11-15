use macros::immutable_event;

fn sample(val: &i32) {
    println!("{}", val)
}

fn sample2(val: &i32) {
    println!("good integer: {}", val)
}

enum Wrapper<'a, T: ?Sized + 'a> {
    Ref(&'a T),
    ToHeap(Box<T>)
}

struct Caller<'a> {
    cs: Vec<Wrapper<'a, dyn Fn(&i32)>>
}

impl<'a> Caller<'a> {
    pub fn new<T: Fn(&i32)>(c: &'a T) -> Self {
        Self {
            cs: vec![Wrapper::Ref(c as &'a dyn Fn(&i32))]
        }
    }

    pub fn add<T: Fn(&i32)>(&mut self, c: &'a T) {
        self.cs.push(Wrapper::Ref(c as &'a dyn Fn(&i32)));
    }

    pub fn add_captures<T: Fn(&i32) + 'a>(&mut self, c: T) {
        self.cs.push(Wrapper::ToHeap(Box::new(c)));
    }

    pub fn call_all(&self, val: &i32) {
        for c in &self.cs {
            match c {
                Wrapper::Ref(r) => r(val),
                Wrapper::ToHeap(b) => b(val)
            }
        }
    }
}

fn main() {
    let mut caller = Caller::new(&sample);
    let i = 2;
    caller.add(&sample2);
    caller.add_captures(move |x| println!("awesome integers: {x}, {i}"));
    caller.call_all(&115);
}
