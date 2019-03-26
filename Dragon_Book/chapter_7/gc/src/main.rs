use gc::GC;

fn main() {
    exercise_7_6_1();
    exercise_7_7_1();
}

fn graph_one() -> GC {
    let mut gc = GC::new();

    let a = gc.alloc("A", 100);
    let b = gc.alloc("B", 100);
    let c = gc.alloc("C", 100);
    let d = gc.alloc("D", 100);
    let e = gc.alloc("E", 100);
    let f = gc.alloc("F", 100);
    let g = gc.alloc("G", 100);
    let h = gc.alloc("H", 100);
    let i = gc.alloc("I", 100);
    let x = gc.alloc_base("X", 100);

    gc.refer(x, a);
    gc.refer(a, b);
    gc.refer(a, c);
    gc.refer(b, d);
    gc.refer(b, e);
    gc.refer(c, f);
    gc.refer(d, g);
    gc.refer(e, c);
    gc.refer(f, h);
    gc.refer(g, e);
    gc.refer(g, h);
    gc.refer(h, i);
    gc.refer(i, g);

    gc
}

fn graph_two() -> GC {
    let mut gc = GC::new();

    let a = gc.alloc("A", 100);
    let b = gc.alloc("B", 100);
    let c = gc.alloc("C", 100);
    let d = gc.alloc("D", 100);
    let e = gc.alloc("E", 100);
    let f = gc.alloc("F", 100);
    let g = gc.alloc("G", 100);
    let h = gc.alloc("H", 100);
    let i = gc.alloc("I", 100);
    let x = gc.alloc_base("X", 100);
    let y = gc.alloc_base("Y", 100);

    gc.refer(x, a);
    gc.refer(y, b);
    gc.refer(a, d);
    gc.refer(a, e);
    gc.refer(b, c);
    gc.refer(b, e);
    gc.refer(c, i);
    gc.refer(d, g);
    gc.refer(d, h);
    gc.refer(d, f);
    gc.refer(e, h);
    gc.refer(f, i);
    gc.refer(g, h);
    gc.refer(h, i);
    gc.refer(i, e);

    gc
}

fn exercise_7_6_1() {
    println!("Exercise 7.6.1:");

    let gc_one = graph_one();
    let gc_two = graph_two();

    let mut instance = gc_one.clone();
    instance.deref(instance.node("A"), instance.node("B"));
    instance.mark_and_sweep(0);
    println!("{:?}", instance);

    let mut instance = gc_one.clone();
    instance.deref(instance.node("A"), instance.node("C"));
    instance.mark_and_sweep(0);
    println!("{:?}", instance);

    let mut instance = gc_two.clone();
    instance.deref(instance.node("A"), instance.node("D"));
    instance.mark_and_sweep(0);
    println!("{:?}", instance);

    let mut instance = gc_two.clone();
    instance.remove(instance.node("B"));
    instance.mark_and_sweep(0);
    println!("{:?}", instance);
}

fn exercise_7_7_1() {
    println!("Exercise 7.7.1:");

    let mut instance = graph_two();
    instance.deref(instance.node("A"), instance.node("D"));
    instance.refer(instance.node("A"), instance.node("H"));
    instance.deref(instance.node("B"), instance.node("C"));
    instance.refer(instance.node("B"), instance.node("I"));
    instance.mark_and_sweep(0);
    println!("{:?}", instance);
}
