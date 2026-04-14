mod engine;

use engine::core::Engine;

fn main() {
    let engine = Engine::new();
    engine.run();
}
