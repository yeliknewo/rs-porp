use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use graphics::{Entity};

pub trait Being {
    fn get_entities(&self) -> &HashMap<u64, Arc<RwLock<Entity>>>;
}
