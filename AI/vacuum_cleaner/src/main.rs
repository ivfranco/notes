use vacuum_cleaner::rectangle::{RandomCleaner, BumpCleaner, Rectangle};
use vacuum_cleaner::simple::{ReflexCleaner, TwoSquare};
use vacuum_cleaner::{simulate, MeasureJudge};


fn main() {
    exercise_2_9();
    exercise_2_11();
}

fn exercise_2_9() {
    println!("2.9");

    for world in TwoSquare::enumerate() {
        for agent in ReflexCleaner::enumerate() {
            print!("World: {:?}, agent: {:?}, ", world, agent);
            println!(
                "Score: {}",
                simulate(world.clone(), agent, MeasureJudge::default())
            );
        }
    }
}

fn exercise_2_11() {
    println!("2.11");

    for _ in 0..5 {
        let world = Rectangle::new(10);
        print!("{:?}", world);
        println!(
            "RandomCleaner Score: {}",
            simulate(world.clone(), RandomCleaner::new(), MeasureJudge::default())
        );
        println!(
            "Stateful BumpCleaner Score: {}",
            simulate(world, BumpCleaner::new(), MeasureJudge::default())
        );
    }
}