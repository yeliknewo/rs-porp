use porp::{init, Window, World, WindowArgs, Game, Vec3, Being};

mod tile;
mod layer;

mod iso_being_type {
    pub enum IsoBeingType {
        Tile,
        Layer,
    }

    use porp::{BeingType};
    impl BeingType for IsoBeingType {}
}

pub use self::iso_being_type::IsoBeingType as IBT;

pub fn main() {
    let manager = init();

    let mut window = Window::new(WindowArgs::Borderless("Iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game = Game::<IBT>::new(manager.clone(), thread_count, resolution);

    let world = game.get_world();

    let world = world.read().expect("Unable to Read World in Main");

    game.run(&mut window);
}
