use crate::Bag;
use lazy_static::lazy_static;
use std::fs::File;

fn open_or_panic(path: &str) -> Bag {
    let file = File::open(path).unwrap();
    Bag::from_reader(file).unwrap()
}

lazy_static! {
    /// Ullman et al., Figure 5.4
    pub static ref PC: Bag = open_or_panic("relations/PC.csv");
    /// Ullman et al., Figure 5.5.a
    pub static ref CLASSES: Bag = open_or_panic("relations/Classes.csv");
    /// Ullman et al., Figure 5.5.b
    pub static ref BATTLES: Bag = open_or_panic("relations/Battles.csv");
    /// Ullman et al., Figure 5.5.c
    pub static ref OUTCOMES: Bag = open_or_panic("relations/Outcomes.csv");
    /// Ullman et al., Figure 5.5.d
    pub static ref SHIPS: Bag = open_or_panic("relations/Ships.csv");
}
