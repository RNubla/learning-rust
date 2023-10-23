pub trait Quack {
    fn quack(&self);
}

pub struct Duck();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

pub struct Bird {
    pub(crate) is_a_parrot: bool,
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

pub struct Animal{
    species: String
}

impl Animal {
    pub fn new(species: String) -> Animal{
        Animal {species}
    }

    pub fn display_species(&self) -> String{
        self.species.to_string()
    }
}

