use scheduling::resource::AluMem;
use scheduling::{Addr, Binary, Code, Delay};

fn main() {
    exercise_10_3_1();
    exercise_10_3_2();
    exercise_10_3_3();
    exercise_10_3_4();
}

fn binary_a() -> Binary {
    Binary::parse(
        "
LD R1, a
LD R2, b
SUB R3, R1, R2
ADD R2, R1, R2
ST a, R3
ST b, R2
    ",
    )
    .unwrap()
}

fn binary_b() -> Binary {
    Binary::parse(
        "
LD R1, a
LD R2, b
SUB R1, R1, R2
ADD R2, R1, R2
ST a, R1
ST b, R2
    ",
    )
    .unwrap()
}

fn binary_c() -> Binary {
    Binary::parse(
        "
LD R1, a
LD R2, b
SUB R3, R1, R2
ADD R4, R1, R2
ST a, R3
ST b, R4
    ",
    )
    .unwrap()
}

fn exercise_10_3_1() {
    println!("Exercise 10.3.1:");
    for binary in &[binary_a(), binary_b(), binary_c()] {
        println!("{:?}", binary.dependency_graph(|_, _| 0));
    }
}

fn elapse(code: &Code) -> Delay {
    use Code::*;
    match code {
        Ld(..) => 2,
        _ => 1,
    }
}

fn delay(earlier: &Code, later: &Code) -> Delay {
    use Addr::*;
    use Code::*;

    match (earlier, later) {
        (Ld(_, Mem(x)), St(Mem(y), _)) if x == y => 1,
        _ => elapse(earlier),
    }
}

fn cost(code: &Code) -> AluMem {
    use Code::*;
    match code {
        Ld(..) => AluMem::new(0, 1),
        St(..) => AluMem::new(0, 1),
        Op(..) => AluMem::new(1, 0),
    }
}

fn report_schedule(binary: &Binary, schedule: &[Delay]) {
    for (i, code) in binary.codes.iter().enumerate() {
        println!("{}: {:?}", schedule[i], code);
    }
}

fn exercise_10_3_2() {
    println!("Exercise 10.3.2:");

    let resources = AluMem::new(1, 1);
    for binary in &[binary_a(), binary_b(), binary_c()] {
        println!();
        let schedule = binary
            .dependency_graph(delay)
            .linear_scheduling(&resources, elapse, cost);
        report_schedule(binary, &schedule);
    }
}

fn exercise_10_3_3() {
    println!("Exercise 10.3.3:");
    for resources in &[AluMem::new(1, 2), AluMem::new(2, 1), AluMem::new(2, 2)] {
        println!("{:?}", resources);
        for binary in &[binary_a(), binary_b(), binary_c()] {
            let schedule = binary
                .dependency_graph(delay)
                .linear_scheduling(resources, elapse, cost);
            report_schedule(binary, &schedule);
            println!();
        }
    }
}

fn figure_10_11() -> Binary {
    Binary::parse(
        "
LD R1, a
ST b, R1
LD R2, c
ST c, R1
LD R1, d
ST d, R2
ST a, R1
    ",
    )
    .unwrap()
}

fn exercise_10_3_4() {
    println!("Exercise 10.3.4:");

    println!("{:?}", figure_10_11().dependency_graph(delay));
}
