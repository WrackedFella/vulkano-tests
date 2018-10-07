extern crate core;

use core::traits::Updateable;

pub struct EventManager {
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            // ToDo: Setup the object and it's structs
        }
    }
}

impl Updateable for EventManager {
    fn update(&self) {
        // ToDo: The needful
        println!("Event Manager");
    }
}
