use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use porp::{Being, Entity, World, Transforms, IDManager, Vec3, RenderUpdateData};

use iso::IBT;

pub struct Layer {
    entities: HashMap<u64, Arc<RwLock<Entity>>>,
    position: Vec3,
    render_updates: Arc<RwLock<RenderUpdateData>>,
}

impl Layer {
    pub fn new(manager: Arc<RwLock<IDManager>>, position: Vec3) -> Box<Layer> {
        Box::new(Layer {
            entities: HashMap::new(),
            position: position,
            render_updates: Arc::new(RwLock::new(RenderUpdateData::new())),
        })
    }
}

impl Being<IBT> for Layer {
    fn get_entities(&self) -> &HashMap<u64, Arc<RwLock<Entity>>> {
        &self.entities
    }

    fn get_render_updates(&self) -> Arc<RwLock<RenderUpdateData>> {
        self.render_updates.clone()
    }

    fn tick_prep(&self, delta_time: &f32, world: &World<IBT>, transforms: &Transforms) {

    }

    fn tick(&mut self, world: Arc<RwLock<World<IBT>>>, transforms: Arc<RwLock<Transforms>>, manager: Arc<RwLock<IDManager>>) {

    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn get_type(&self) -> IBT {
        IBT::Layer
    }
}
