use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use glium::glutin::Event as WindowEvent;
use glium::glutin::ElementState as GliumElementState;
use glium::glutin::MouseButton as GliumMouseButton;
use glium::glutin::VirtualKeyCode as GliumKeyCode;
use scoped_threadpool::{Pool};
use time::{precise_time_s};

use ids::{ID, IDManager, IDType};
use world::{World};
use graphics::{Window, Transforms, method_to_parameters};
use being::{Being};
use math::{Vec2};
use keyboard::{Keyboard};

pub struct Game {
    world: Arc<RwLock<World>>,
    thread_pool: Pool,
    resolution: Vec2,
    aspect_ratio: f32,
    mouse_pos: Vec2,
    keyboard: Arc<RwLock<Keyboard>>,
    mouse_buttons: HashMap<GliumMouseButton, GliumElementState>,
    transforms: Arc<RwLock<Transforms>>,
    manager: Arc<RwLock<IDManager>>,
}

impl Game {
    pub fn new(manager: IDManager, thread_count: u32, resolution: Vec2) -> Game {
        Game {
            world: Arc::new(RwLock::new(World::new())),
            thread_pool: Pool::new(thread_count),
            aspect_ratio: resolution[0] / resolution[1],
            resolution: resolution,
            mouse_pos: Vec2::zero(),
            keyboard: Arc::new(RwLock::new(Keyboard::new())),
            mouse_buttons: HashMap::new(),
            transforms: Arc::new(RwLock::new(Transforms::new())),
            manager: Arc::new(RwLock::new(manager)),
        }
    }

    fn pause(&mut self) {
        println!("Paused");
    }

    fn resume(&mut self) {
        println!("Resumed");
    }

    fn update_keyboard(&mut self, key_code: GliumKeyCode, element_state: GliumElementState) {
        self.keyboard.set_key_state(key_code, element_state);
        self.world.write().expect("Unable to Write World in Update Mouse Pos").update_keyboard(key_code, element_state);
    }

    fn update_mouse_button(&mut self, mouse_button: GliumMouseButton, element_state: GliumElementState, ) {
        self.mouse_buttons.insert(mouse_button, element_state);
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Mouse Button").write().expect("Unable to Write Active World in Update Mouse Button").update_mouse_button(mouse_button, element_state);
    }

    fn update_mouse_pos(&mut self, mouse_pos: (i32, i32)) {
        let x = mouse_pos.0 as f32;
        let y = mouse_pos.1 as f32;
        self.mouse_pos = Vec2::from([x, y]);
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Mouse Pos").write().expect("Unable to Write Active World in Update Mouse Pos").update_mouse_pos(self.mouse_pos);
    }

    fn update_resolution(&mut self, resolution: (u32, u32)) {
        let width = resolution.0 as f32;
        let height = resolution.1 as f32;
        self.resolution = Vec2::from([width, height]);
        self.aspect_ratio = width / height;
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Resolution").write().expect("Unable to Write Active World in Update Resolution").update_resolution(self.resolution, self.aspect_ratio);
    }

    pub fn run(&mut self, window: &mut Window) {
        let tps: f64 = 60.0;
        let tps_s: f64 = 1.0 / tps;

        let mut last_time: f64 = precise_time_s();
        let mut delta_time: f64 = 0.0;

        let mut i: f64 = last_time;

        let mut frames: u64 = 0;
        let mut ticks: u64 = 0;

        loop {
            let now = precise_time_s();
            delta_time += now - last_time;
            last_time = now;
            while delta_time > 0.0 {
                for event in window.poll_events(){
                    match event {
                        WindowEvent::Resized(width, height) => self.update_resolution((width, height)),
                        // WindowEvent::Moved(x, y) => {
                        //
                        // },
                        WindowEvent::Closed => return,
                        // WindowEvent::DroppedFile(path_buffer) => {
                        //
                        // },
                        // WindowEvent::ReceivedCharacter(character) => {
                        //
                        // },
                        WindowEvent::Focused(focused) => {
                            if focused {
                                self.resume();
                            } else {
                                self.pause();
                            }
                        },
                        WindowEvent::KeyboardInput(element_state, _, virtual_key_code) => match virtual_key_code {
                            Some(virtual_key_code) => self.update_keyboard(virtual_key_code, element_state),
                            None => (),
                        },
                        WindowEvent::MouseMoved(pos) => self.update_mouse_pos(pos),
                        // WindowEvent::MouseWheel(mouse_scroll_data) => {
                        //
                        // },
                        WindowEvent::MouseInput(element_state, mouse_button) => self.update_mouse_button(mouse_button, element_state),
                        // WindowEvent::Awakened => {
                        //
                        // },
                        // WindowEvent::Refresh => {
                        //
                        // },
                        // WindowEvent::Suspended(suspended) => {
                        //
                        // },
                        // WindowEvent::Touch(touch) => {
                        //
                        // },
                        _ => (),
                    }
                }
                {
                    let tps_f32 = tps_s as f32;
                    let events = self.tick(tps_f32);
                }
                delta_time -= tps_s;
                ticks += 1;
            }
            self.render(window);
            frames += 1;
            if now > i + 1.0 {
                i += 1.0;
                println!("{} {}", frames.to_string(), ticks.to_string());
                frames = 0;
                ticks = 0;
            }
        }
    }

    fn render(&mut self, window: &mut Window) {
        let mut frame = window.frame();
        for entry in self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Render").read().expect("Unable to Read World when rendering").get_beings() {
            let being = entry.1;
            for entity in being.read().expect("Unable to Read Being when rendering").get_entities() {
                frame.draw_entity(entity.1, &self.transforms);
            }
        }
        frame.end();
    }

    fn tick(&mut self, delta_time: f32) {

    }
}
