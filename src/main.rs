extern crate core;
extern crate scene_manager;

use std::path::Path;

use scene_manager::SceneManager;
use core::traits::Updateable;

fn main() {
    let path = Path::new("C:\\dev\\samples\\3d\\house\\model.obj");
    let mut manager = SceneManager::new();
    manager.load_file(path);
    // let mut managers = init_managers();
    // let mut i = 10;
    // while i > 0 {
    //     managers.iter_mut().for_each(|m| { m.update() });
    //     i = i - 1;
    // }
}

fn init_managers() -> Vec<Box<Updateable>> {
    let mut managers: Vec<Box<Updateable>> = Vec::new();
    managers.push(Box::new(SceneManager::new()));
    return managers;
}

