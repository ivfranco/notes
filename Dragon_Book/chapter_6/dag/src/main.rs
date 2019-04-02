use dag::env::Env;
use dag::symbolic;
use dag::symbolic::three_addr::ProcBuilder;
use dag::utils::Array;

fn main() {
    exercise_6_1_1();
    exercise_6_1_2();
    exercise_6_2_1();
    exercise_6_3_1();
    exercise_6_4_3();
    exercise_6_4_6();
    exercise_6_4_7();
    exercise_6_4_8();
    exercise_6_4_9();
    exercise_6_6_4();
    exercise_8_4_1();
    exercise_8_4_2();
    exercise_8_5_8();
    exercise_8_6_1();
}

fn exercise_6_1_1() {
    println!("Exercise 6.1.1:");

    println!(
        "{:?}",
        symbolic::DAG::parse("((x + y) - ((x + y) * (x - y))) + ((x + y) * (x - y))").unwrap()
    );
}

fn exercise_6_1_2() {
    println!("Exercise 6.1.2:");

    println!("{:?}", symbolic::DAG::parse("a + b + (a + b)").unwrap());
    println!("{:?}", symbolic::DAG::parse("a + b + a + b").unwrap());
    println!(
        "{:?}",
        symbolic::DAG::parse("a + a + (a + a + a + (a + a + a + a))").unwrap()
    );
}

fn exercise_6_2_1() {
    println!("Exercise 6.2.1:");

    println!("{:?}", symbolic::DAG::parse("a + -(b + c)").unwrap());
}

fn exercise_6_3_1() {
    println!("Exercise 6.3.1:");

    let decls = "float x;
record { float x; float y; } p;
record { int tag; float x; float y; } q;";

    println!("{:?}", Env::parse(decls).unwrap());
}

fn exercise_6_4_3() {
    println!("Exercise 6.4.3:");

    println!("{:#?}", ProcBuilder::parse("x = a[i] + b[j];").unwrap());
    println!(
        "{:#?}",
        ProcBuilder::parse("x = a[i][j] + b[i][j];").unwrap()
    );
    println!(
        "{:#?}",
        ProcBuilder::parse("x = a[b[i][j]][c[k]];").unwrap()
    );
}

fn exercise_6_4_6() {
    println!("Exercise 6.4.6:");

    let arr = Array::new(0, 4, &[(1, 10), (1, 20)]);
    println!("{}", arr.row_major(&[4, 5]));
    println!("{}", arr.row_major(&[10, 8]));
    println!("{}", arr.row_major(&[3, 17]));
}

fn exercise_6_4_7() {
    println!("Exercise 6.4.7:");

    let arr = Array::new(0, 4, &[(1, 10), (1, 20)]);
    println!("{}", arr.col_major(&[4, 5]));
    println!("{}", arr.col_major(&[10, 8]));
    println!("{}", arr.col_major(&[3, 17]));
}

fn exercise_6_4_8() {
    println!("Exercise 6.4.8:");

    let arr = Array::new(0, 8, &[(1, 4), (0, 4), (5, 10)]);
    println!("{}", arr.row_major(&[3, 4, 5]));
    println!("{}", arr.row_major(&[1, 2, 7]));
    println!("{}", arr.row_major(&[4, 3, 9]));
}

fn exercise_6_4_9() {
    println!("Exercise 6.4.9:");

    let arr = Array::new(0, 8, &[(1, 4), (0, 4), (5, 10)]);
    println!("{}", arr.col_major(&[3, 4, 5]));
    println!("{}", arr.col_major(&[1, 2, 7]));
    println!("{}", arr.col_major(&[4, 3, 9]));
}

fn exercise_6_6_4() {
    println!("Exercise 6.6.4");

    println!(
        "{:#?}",
        ProcBuilder::parse("if (a==b && c==d || e==f) { x = 1; }").unwrap()
    );
    println!(
        "{:#?}",
        ProcBuilder::parse("if (a==b || c==d || e==f) { x = 1; }").unwrap()
    );
    println!(
        "{:#?}",
        ProcBuilder::parse("if (a==b && c==d && e==f) { x = 1; }").unwrap()
    );
}

fn exercise_8_4_1() {
    println!("Exercise 8.4.1:");

    println!(
        "{:#?}",
        ProcBuilder::parse(
            "
for (i=0; i<n; i++) {
    for (j=0; j<n; j++) {
        c[i][j] = 0.0;
    }
}
for (i=0; i<n; i++) {
    for (j=0; j<n; j++) {
        for (k=0; k<n; k++) {
            c[i][j] = c[i][j] + a[i][k]*b[k][j];
        }
    }
}
    "
        )
        .unwrap()
    );
}

fn exercise_8_4_2() {
    println!("Exercise 8.4.2:");

    println!(
        "{:#?}",
        ProcBuilder::parse(
            "
for (i=2; i<=n; i++) {
    a[i] = TRUE;
}
count = 0;
s = 0;
for (i=2; i<=s; i++) {
    if (a[i] == FALSE) {
        count++;
        for (j=2*i; j<=n; j = j+i) {
            a[j] = FALSE;
        }
    }
}
    "
        )
        .unwrap()
    );
}

fn exercise_8_5_8() {
    println!("Exercise 8.5.8:");

    println!(
        "{:#?}",
        ProcBuilder::parse(
            "x = a + b + c + d + e + f;
y = a + c + e;"
        )
        .unwrap()
    );
}

fn exercise_8_6_1() {
    println!("Exercise 8.6.1:");

    println!("{:#?}", ProcBuilder::parse("x = a + b*c;").unwrap());
    println!(
        "{:#?}",
        ProcBuilder::parse("x = a/(b+c) - d*(e+f);").unwrap()
    );
    println!("{:#?}", ProcBuilder::parse("x = a[i] + 1;").unwrap());
    println!("{:#?}", ProcBuilder::parse("a[i] = b[c[i]];").unwrap());
    println!(
        "{:#?}",
        ProcBuilder::parse("a[i][j] = b[i][k] + c[k][j];").unwrap()
    );
}
