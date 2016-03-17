use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use porp::{Being, Entity, World, Transforms, IDManager, Vec3, RenderUpdateData};

pub const ENTITY_TILE: u64 = 0;

pub struct Tile {
    entities: HashMap<u64, Arc<RwLock<Entity>>>,
    position: Vec3,
    render_updates: Arc<RwLock<RenderUpdateData>>,
}

impl Tile {
    pub fn new(manager: Arc<RwLock<IDManager>>, position: Vec3) -> Tile {
        let mut entities = HashMap::new();
        entities.insert(ENTITY_TILE, Arc::new(RwLock::new(Entity::new(manager))));
        let mut render_updates = RenderUpdateData::new();
        Tile {
            entities: entities,
            position: position,
            render_updates: Arc::new(RwLock::new(render_updates)),
        }
    }
}

impl Being for Tile {
    fn get_entities(&self) -> &HashMap<u64, Arc<RwLock<Entity>>> {
        &self.entities
    }

    fn get_render_updates(&self) -> Arc<RwLock<RenderUpdateData>> {
        self.render_updates.clone()
    }

    fn tick_prep(&self, delta_time: &f32, world: &World, transforms: &Transforms) {

    }

    fn tick(&mut self, world: Arc<RwLock<World>>, transforms: Arc<RwLock<Transforms>>, manager: Arc<RwLock<IDManager>>) {

    }

    fn get_position(&self) -> Vec3 {
        self.position
    }
}
