extern crate core;

use core::traits::Updateable;

pub struct ObjectManager {
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager {
            // ToDo: Setup the object and it's structs
        }
    }
}

impl Updateable for ObjectManager {
    fn update(&self) {
        // ToDo: The needful
        println!("Object Manager");
    }
}
