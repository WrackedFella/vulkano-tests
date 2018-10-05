extern crate core;
extern crate scene_manager;
extern crate object_manager;
extern crate event_manager;

use scene_manager::SceneManager;
use object_manager::ObjectManager;
use event_manager::EventManager;

use core::traits::Updateable;

fn main() {
    let mut managers: Vec<Box<Updateable>> = Vec::new();
    managers.push(Box::new(SceneManager::new()));
    managers.push(Box::new(ObjectManager::new()));
    managers.push(Box::new(EventManager::new()));
    let mut i = 10;
    while i > 0 {
        managers.iter_mut().for_each(|m| { m.update() });
        i = i - 1;
    }
}
