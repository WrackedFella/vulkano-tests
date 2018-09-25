extern crate cgmath;
extern crate winit;
extern crate time;
extern crate core;

use core::traits::Updateable;

pub struct SceneManager {
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            // ToDo: Setup the object and it's structs
        }
    }
}

impl Updateable for SceneManager {
    fn update(&self) {
        // ToDo: The needful
        println!("Hello, world!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
