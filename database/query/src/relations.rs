use crate::Bag;
use lazy_static::lazy_static;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

fn open_or_panic<P: AsRef<Path>>(path: P) -> Bag {
    let file = File::open(path).unwrap();
    Bag::from_reader(file).unwrap()
}

lazy_static! {
    /// location of CSV files
    pub static ref RELATIONS: PathBuf = PathBuf::from("relations");
    /// Ullman et al., Figure 5.4
    pub static ref PC: Bag = open_or_panic(RELATIONS.join("PC.csv"));
    /// Ullman et al., Figure 5.5.a
    pub static ref CLASSES: Bag = open_or_panic(RELATIONS.join("Classes.csv"));
    /// Ullman et al., Figure 5.5.b
    pub static ref BATTLES: Bag = open_or_panic(RELATIONS.join("Battles.csv"));
    /// Ullman et al., Figure 5.5.c
    pub static ref OUTCOMES: Bag = open_or_panic(RELATIONS.join("Outcomes.csv"));
    /// Ullman et al., Figure 5.5.d
    pub static ref SHIPS: Bag = open_or_panic(RELATIONS.join("Ships.csv"));
}
