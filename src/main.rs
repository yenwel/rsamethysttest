extern crate amethyst;

use amethyst::prelude::*;

struct HelloWorld;

impl State for HelloWorld {
    fn on_start(&mut self, _: &mut Engine) {
        println!("Game started!");
    }

    fn update(&mut self, _: &mut Engine) -> Trans {
        println!("Hello from Amethyst!");
        Trans::Quit
    }

    fn on_stop(&mut self, _: &mut Engine) {
        println!("Game stopped!");
    }
}

fn main() {
    //let cfg = Config::default();
    let mut game = Application::build(HelloWorld/*, cfg**/).unwrap().build()./*finish().*/expect("Fatal");
    game.run();
}