use inference::{unify, Term};

fn main() {
    exercise_9_4();
}

fn exercise_9_4() {
    println!("9.4");

    for (s0, s1) in &[
        ("P(A,B,B)", "P(x,y,z)"),
        ("Q(y,G(A,B))", "Q(G(x,x),y)"),
        ("Older(Father(y),y)", "Older(Father(x),John)"),
        ("Knows(Father(y),y)", "Knows(x,x)"),
    ] {
        let s0 = Term::parse(s0).into();
        let s1 = Term::parse(s1).into();
        let unifier = unify(&s0, &s1);
        println!("{:?}", unifier);

        if let Some(unifier) = unifier {
            assert_eq!(s0.subst(&unifier), s1.subst(&unifier));
        }
    }
}
