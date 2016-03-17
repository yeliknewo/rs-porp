#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;
extern crate rand;
extern crate num;

use std::sync::{Arc, RwLock};

mod math;
mod input;
mod logic;
mod graphics;
mod utils;

pub use self::math::{Mat4, Vec2, Vec3, Vec4};
pub use self::input::{Keyboard, Mouse, Display};
pub use self::graphics::{DrawMethod, Window, Frame, WindowArgs, Transforms, Entity, Vertex, Index, init_vertex, ID, IDType, IDManager};
pub use self::logic::{Being, BeingType, Game, World, RenderUpdateData};
pub use self::utils::{KeyCode, ButtonState, MouseButton};

pub fn init() -> Arc<RwLock<IDManager>> {
    graphics::init_vertex();
    Arc::new(RwLock::new(IDManager::new()))
}
