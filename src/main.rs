extern crate core;
extern crate scene_manager;

use scene_manager::SceneManager;
use core::traits::Updateable;

fn main() {
    let scene_manager: SceneManager = SceneManager::new();
    let mut v: Vec<Box<Updateable>> = Vec::new();
    v.push(Box::new(scene_manager));
    for manager in v.iter() {
        manager.update();
    }
}
