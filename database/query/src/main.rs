use query::Bag;
use query::{relations, Atom};
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};

fn main() {
    exercise_5_1_1();
    exercise_5_1_2();
    exercise_5_1_3();
}

fn avg(bag: &Bag) -> Decimal {
    bag.aggregate(Decimal::zero(), |sum, atom, cnt| {
        if let Atom::Number(d) = atom {
            d * Decimal::from_usize(cnt).unwrap() + sum
        } else {
            sum
        }
    }) / Decimal::from_usize(bag.len()).unwrap()
}

fn exercise_5_1_1() {
    println!("\nexercise 5.1.1");
    let bag = relations::PC.project(["speed"]);
    let set = bag.dedup();

    println!("bag:\n{}", bag);
    println!("set:\n{}", set);
    println!("avg of bag: {:.2}", avg(&bag));
    println!("avg of set: {:.2}", avg(&set));
}

fn exercise_5_1_2() {
    println!("\nexercise 5.1.2");
    let bag = relations::PC.project(["hd"]);
    let set = bag.dedup();

    println!("bag:\n{}", bag);
    println!("set:\n{}", set);
    println!("avg of bag: {:.2}", avg(&bag));
    println!("avg of set: {:.2}", avg(&set));
}

fn exercise_5_1_3() {
    println!("\nexercise 5.1.3");

    let bag = relations::CLASSES.project(["bore"]);
    let set = bag.dedup();

    println!("bag:\n{}", bag);
    println!("set:\n{}", set);
}
