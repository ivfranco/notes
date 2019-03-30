use naive::machine_code::Binary;
use naive::three_addr::Program;

fn main() {
    exercise_8_2_1();
    exercise_8_2_2();
    exercise_8_2_3();
    exercise_8_2_4();
    exercise_8_2_5();
    exercise_8_2_6();
    exercise_8_3_2();
    exercise_8_3_3();
}

fn exercise_8_2_1() {
    println!("Exercise 8.2.1:");

    println!("{:?}", Program::parse("x = 1;").unwrap().build());
    println!("{:?}", Program::parse("x = a;").unwrap().build());
    println!("{:?}", Program::parse("x = a + 1;").unwrap().build());
    println!("{:?}", Program::parse("x = a + b;").unwrap().build());
    println!(
        "{:?}",
        Program::parse("x = b * c; y = a + x;").unwrap().build()
    );
}

fn exercise_8_2_2() {
    println!("Exercise 8.2.2:");

    println!(
        "{:?}",
        Program::parse(
            "
x = a[i];
y = b[j];
a[i] = y;
b[j] = x;
"
        )
        .unwrap()
        .build()
    );

    println!(
        "{:?}",
        Program::parse(
            "
x = a[i];
y = b[i];
z = x * y;
"
        )
        .unwrap()
        .build()
    );

    println!(
        "{:?}",
        Program::parse(
            "
x = a[i];
y = b[x];
a[i] = y;
"
        )
        .unwrap()
        .build()
    );
}

fn exercise_8_2_3() {
    println!("Exercise 8.2.3:");

    let program = "y = *q;
q = q + 4;
*p = y;
p = p + 4;";

    println!("{:?}", Program::parse(program).unwrap().build());
}

fn exercise_8_2_4() {
    println!("Exercise 8.2.4:");

    let program = "if x < y goto L1;
z = 0;
goto L2;
L1: z = 1;";

    println!("{:?}", Program::parse(program).unwrap().build());
}

fn exercise_8_2_5() {
    println!("Exercise 8.2.5:");

    let program = "
s = 0;
i = 0;
L1: if i > n goto L2;
s = s + i;
i = i + 1;
goto L1;
L2:;";

    println!("{:?}", Program::parse(program).unwrap().build());
}

fn exercise_8_2_6() {
    println!("Exercise 8.2.6:");

    println!(
        "{:?}",
        Binary::parse(
            "
LD R0, y
LD R1, z
ADD R0, R0, R1
ST x, R0
"
        )
        .unwrap()
        .cost()
    );

    println!(
        "{:?}",
        Binary::parse(
            "
LD R0, i
MUL R0, R0, 8
LD R1, a(R0)
ST b, R1
"
        )
        .unwrap()
        .cost()
    );

    println!(
        "{:?}",
        Binary::parse(
            "
LD R0, c
LD R1, i
MUL R1, R1, 8
ST a(R1), R0
"
        )
        .unwrap()
        .cost()
    );

    println!(
        "{:?}",
        Binary::parse(
            "
LD R0, p
LD R1, 0(R0)
ST x, R1
"
        )
        .unwrap()
        .cost()
    );

    println!(
        "{:?}",
        Binary::parse(
            "
LD R0, p
LD R1, x
ST 0(R0), R1
"
        )
        .unwrap()
        .cost()
    );

    println!(
        "{:?}",
        Binary::parse(
            "
LD R0, x
LD R1, y
SUB R0, R0, R1
BLTZ *R3, L0
"
        )
        .unwrap()
        .cost()
    );
}

fn exercise_8_3_2() {
    println!("Exercise 8.3.2:");

    println!("{}", Program::parse("x = 1;").unwrap().build());
    println!("{}", Program::parse("x = a;").unwrap().build());
    println!("{}", Program::parse("x = a + 1;").unwrap().build());
    println!("{}", Program::parse("x = a + b;").unwrap().build());
    println!(
        "{}",
        Program::parse("x = b * c; y = a + x;").unwrap().build()
    );
}

fn exercise_8_3_3() {
    println!("Exercise 8.3.3:");

    println!(
        "{}",
        Program::parse(
            "
x = a[i];
y = b[j];
a[i] = y;
b[j] = x;
    "
        )
        .unwrap()
        .build()
    );

    println!(
        "{}",
        Program::parse(
            "
x = a[i];
y = b[i];
z = x * y;
    "
        )
        .unwrap()
        .build()
    );

    println!(
        "{}",
        Program::parse(
            "
x = a[i];
y = b[x];
a[i] = y;
    "
        )
        .unwrap()
        .build()
    );
}
