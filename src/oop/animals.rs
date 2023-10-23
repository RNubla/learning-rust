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

pub struct Vehicle{
    charge: Option<u32>,
    make: String,
    is_car: bool,    // if true -> its a car else its a truck
}

impl Vehicle {
    pub fn new(charge: Option<u32>, make: String, is_car : bool) -> Vehicle {
        Vehicle { charge: charge, make, is_car}
    }
}

pub trait Electric{
    fn set_charge(&mut self,charge_percentage: u32 );
    // fn get_charge(&self) -> u32;
    fn get_charge(&self) -> Result<u32, &'static str>;
    // fn get_range(&self) -> u32;
    fn need_to_charge(&self) -> bool {
        match self.get_charge() {
            Ok(charge) => charge <= 20,
            Err(e) => {
                println!("Error checking charge: {}", e);
                false // Assuming no need to charge if there's an error, or true if you want to charge on errors
            },
        }
    }
}

impl Electric for Vehicle{
    fn set_charge(&mut self, charge_percentage: u32) {
        self.charge = Some(charge_percentage);
    }
    // handling error with unwrap will crash the program if the Option is None
    // fn get_charge(&self) -> u32 {
    //     return self.charge.unwrap();
    // }
    fn get_charge(&self) -> Result<u32, &'static str>{
        match self.charge{
            Some(charge) => Ok(charge),
            None => Err("Charge not set for this vehicle.")
        }
    }
}