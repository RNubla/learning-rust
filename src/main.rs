mod oop;
use crate::oop::animals::{Animal, Bird, Duck, Quack};
use crate::oop::shape::{Circle, Rectangle, Shape, Triangle};
use crate::oop::vehicles::{Vehicle,Electric};
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

    let human = Animal::new("Human".to_string());
    println!("{}",human.display_species());

    let cat = Animal::new("Cat".to_string());
    println!("{}",cat.display_species());

    // let honda_civic = Vehicle::new("Honda".to_string(), true);
    let mut tesla = Vehicle::new(Option::from(4u32), "Tesla".to_string(), true);


    if tesla.need_to_charge(){
        println!("The car needs to be charged")
    }else{
        println!("The car is sufficiently charged")
    }
    tesla.set_charge(25u32);
    if tesla.need_to_charge(){
        println!("The car needs to be charged")
    }else{
        println!("The car is sufficiently charged")
    }

    let triangle = Circle::new(4.00).area();
    println!("{}", triangle)

}
