use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use math::{Vec3};
use graphics::{Entity, Transforms, IDManager, Window, Vertex, Index, DrawMethod, method_to_parameters};
use logic::{World};

pub trait BeingType { }

pub trait Being<T: BeingType> : Send + Sync {
    fn get_entities(&self) -> &HashMap<u64, Arc<RwLock<Entity>>>;
    fn get_render_updates(&self) -> Arc<RwLock<RenderUpdateData>>;
    fn tick_prep(&self, &f32, &World<T>, &Transforms);
    fn tick(&mut self, Arc<RwLock<World<T>>>, Arc<RwLock<Transforms>>, Arc<RwLock<IDManager>>);
    fn render(&mut self, window: &mut Window) {
        self.update_vertices(window);
        self.update_indices(window);
    }
    fn get_type(&self) -> T;
    fn get_position(&self) -> Vec3;
    fn update_vertices(&self, window: &mut Window) {
        let updates = self.get_render_updates();
        let mut updates = updates.write().expect("Unable to Write Render Updates in Update Vertices in Being");
        let length = updates.vertices.len();
        for entry in updates.vertices.drain(0..length) {
            window.set_vertices(self.get_entities().get(&entry.0).expect("Unable to Get Entity in Update Vertices in Being"), entry.1);
        }
    }
    fn update_indices(&self, window: &mut Window) {
        let updates = self.get_render_updates();
        let mut updates = updates.write().expect("Unable to Write Render Updates in Update Indices in Being");
        let length = updates.indices.len();
        for entry in updates.indices.drain(0..length) {
            window.set_indices(self.get_entities().get(&entry.0).expect("Unable to Get Entity in Update Indices in Being"), entry.1);
        }
    }
    fn update_texture(&self, window: &mut Window) {
        let updates = self.get_render_updates();
        let mut updates = updates.write().expect("Unable to Write Render Updates in Update Texture in Being");
        let length = updates.texture.len();
        for entry in updates.texture.drain(0..length) {
            window.set_texture(self.get_entities().get(&entry.0).expect("Unable to Get Entity in Update Texture in Being"), entry.1);
        }
    }
    fn update_draw_method(&self, window: &mut Window) {
        let updates = self.get_render_updates();
        let mut updates = updates.write().expect("Unable to Write Render Updates in Update Draw Method in Being");
        let length = updates.draw_method.len();
        for entry in updates.draw_method.drain(0..length) {
            window.set_draw_parameters(self.get_entities().get(&entry.0).expect("Unable to Get Entity in Update Texture in Being"), method_to_parameters(entry.1));
        }
    }
}

pub struct RenderUpdateData {
    vertices: Vec<(u64, Vec<Vertex>)>,
    indices: Vec<(u64, Vec<Index>)>,
    texture: Vec<(u64, &'static [u8])>,
    draw_method: Vec<(u64, DrawMethod)>,
}

impl RenderUpdateData {
    pub fn new() -> RenderUpdateData {
        RenderUpdateData {
            vertices: vec!(),
            indices: vec!(),
            texture: vec!(),
            draw_method: vec!(),
        }
    }
}
