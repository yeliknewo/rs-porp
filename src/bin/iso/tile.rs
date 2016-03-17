use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use porp::{Being, Entity, World, Transforms, IDManager, Vec3, RenderUpdateData, IDType};

use iso::IBT;

pub const ENTITY_TILE: u64 = 0;

pub struct Tile {
    entities: HashMap<u64, Arc<RwLock<Entity>>>,
    position: Vec3,
    render_updates: Arc<RwLock<RenderUpdateData>>,
}

impl Tile {
    pub fn new(manager: Arc<RwLock<IDManager>>, position: Vec3) -> Box<Tile> {
        let mut entities = HashMap::new();
        entities.insert(ENTITY_TILE, Arc::new(RwLock::new(Entity::new(manager))));
        let mut render_updates = RenderUpdateData::new();
        Box::new(Tile {
            entities: entities,
            position: position,
            render_updates: Arc::new(RwLock::new(render_updates)),
        })
    }

    pub fn new_from(manager: Arc<RwLock<IDManager>>, position: Vec3, base: Box<Tile>) -> Box<Tile> {
        let mut entities = HashMap::new();
        let mut render_updates = RenderUpdateData::new();
        let their_entities = base.get_entities();
        for entry in their_entities.iter() {
            let mut entity = Entity::new_from(entry.1);
            entity.use_new_id(manager.clone(), IDType::Model);
            entities.insert(*entry.0, Arc::new(RwLock::new(entity)));
        }
        Box::new(Tile{
            entities: entities,
            position: position,
            render_updates: Arc::new(RwLock::new(render_updates)),
        })
    }
}

impl Being<IBT> for Tile {
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
        IBT::Tile
    }
}
