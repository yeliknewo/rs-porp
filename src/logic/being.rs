use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use graphics::{Entity, Transforms, IDManager};
use logic::{World};

pub trait Being : Send + Sync {
    fn get_entities(&self) -> &HashMap<u64, Arc<RwLock<Entity>>>;
    fn tick_prep(&self, &f32, &World, &Transforms);
    fn tick(&mut self, Arc<RwLock<World>>, Arc<RwLock<Transforms>>, Arc<RwLock<IDManager>>);
}
