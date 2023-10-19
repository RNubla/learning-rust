trait Quack {
    fn quack(&self);
}

struct Duck();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

struct Bird {
    is_a_parrot: bool,
}

impl Quack for Bird {
    fn quack(&self) {
        if !self.is_a_parrot {
            println!("quack")
        } else {
            println!("sqwak")
        }
    }
}

fn main() {
    /***
     * The difference is that you can now take this vector, pass it as a reference or give it away without having to track any borrowed references. When the vector is dropped, the boxes will be dropped, and all memory is reclaimed.
     */
    let duck1 = Box::new(Duck());
    let duck2 = Box::new(Bird { is_a_parrot: false });
    let parrot = Box::new(Bird { is_a_parrot: true });

    let ducks: Vec<Box<dyn Quack>> = vec![duck1, duck2, parrot];

    for d in &ducks {
        d.quack();
    }
}
