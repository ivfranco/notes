use free_space::Heap;

fn main() {
    exercise_7_4_1();
}

fn exercise_7_4_1() {
    let sizes = &[80, 30, 60, 50, 70, 20, 40];
    let objects = &[32, 64, 48, 16];

    let mut heap = Heap::new(sizes);
    for object in objects {
        heap.first_fit(*object);
    }
    println!("{:?}", heap);

    let mut heap = Heap::new(sizes);
    for object in objects {
        heap.best_fit(*object);
    }
    println!("{:?}", heap);
}
