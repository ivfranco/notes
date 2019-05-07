use loop_nest::utils::gcd;
use loop_nest::Triple;

fn main() {
    exercise_11_3_3();
    exercise_11_3_4();
    exercise_11_3_5();
    exercise_11_3_6();
    exercise_11_3_7();
    exercise_11_6_1();
}

fn loop_nest_a() -> Triple {
    Triple::new(
        &['i', 'j'],
        &[
            (&[1], &[0, 1]),
            (&[0, 1], &[29]),
            (&[2, 1], &[0, 0, 1]),
            (&[0, 0, 1], &[39, -1]),
        ],
    )
}

fn loop_nest_b() -> Triple {
    Triple::new(
        &['i', 'j'],
        &[
            (&[10], &[0, 1]),
            (&[0, 1], &[1000]),
            (&[0, 1], &[0, 0, 1]),
            (&[0, 0, 1], &[9, 1]),
        ],
    )
}

fn loop_nest_c() -> Triple {
    Triple::new(
        &['i', 'j', 'k'],
        &[
            (&[0], &[0, 1]),
            (&[0, 1], &[99]),
            (&[0], &[0, 0, 1]),
            (&[0, 0, 1], &[99, 1]),
            (&[0, 1, 1], &[0, 0, 0, 1]),
            (&[0, 0, 0, 1], &[99, -1, -1]),
        ],
    )
}

fn exercise_11_3_3() {
    println!("Exercise 11.3.3:");

    println!("{:?}", loop_nest_a());
    println!("{:?}", loop_nest_b());
    println!("{:?}", loop_nest_c());
}

fn exercise_11_3_4() {
    println!("Exercise 11.3.4:");

    loop_nest_a().report_constraints(&['i', 'j']);
    println!();
    loop_nest_b().report_constraints(&['i', 'j']);
    println!();
    loop_nest_c().report_constraints(&['i', 'j', 'k']);
}

fn exercise_11_3_5() {
    println!("Exercise 11.3.5:");

    for loop_nest in &[loop_nest_a(), loop_nest_b(), loop_nest_c()] {
        println!("{:?}", loop_nest.eliminate('i').unwrap());
    }
}

fn exercise_11_3_6() {
    println!("Exercise 11.3.6:");

    for loop_nest in &[loop_nest_a(), loop_nest_b(), loop_nest_c()] {
        println!("{:?}", loop_nest.eliminate('j').unwrap());
    }
}

fn exercise_11_3_7() {
    println!("Exercise 11.3.7:");

    let triple = Triple::new(
        &['j', 'k'],
        &[
            (&[1], &[0, 1, -1]),
            (&[0, 1, -1], &[29]),
            (&[2, 1, -1], &[0, 1]),
            (&[0, 1], &[39, -1, 1]),
        ],
    );

    triple.report_constraints(&['j', 'k']);
}

fn exercise_11_6_1() {
    println!("Exercise 11.6.1:");

    println!("{}", gcd(&[16, 24, 56]));
    println!("{}", gcd(&[-45, 105, 240]));
    println!("{}", gcd(&[84, 105, 180, 315, 350]));
}
