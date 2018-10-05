extern crate core;
extern crate scene_manager;
extern crate object_manager;

use scene_manager::SceneManager;
use object_manager::ObjectManager;
use core::traits::Updateable;

fn main() {
    let mut v: Vec<Box<Updateable>> = Vec::new();
    v.push(Box::new(SceneManager::new()));
    v.push(Box::new(ObjectManager::new()));
    let mut i = 10;
    while i > 0 {
        v.iter_mut().for_each(|manager| { manager.update() });
        i = i - 1;
    }
}
