use vaccum_cleaner::simple::{ReflexCleaner, TwoSquare};
use vaccum_cleaner::simulate;

fn main() {
    exercise_2_9();
}

fn exercise_2_9() {
    println!("2.9");

    for world in TwoSquare::enumerate() {
        for agent in ReflexCleaner::enumerate() {
            print!("World: {:?}, agent: {:?}, ", world, agent);
            println!("Score: {}", simulate(world.clone(), agent));
        }
    }
}
