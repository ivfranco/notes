use inference::{unify_term, Term, Unifier};

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
        // cheating here by parsing s0 and s1 as terms
        // a full first-order logic sentence parser would require some LR parser generator
        let t0 = Term::parse(s0);
        let t1 = Term::parse(s1);
        let unifier = unify_term(&t0, &t1, Unifier::new());
        println!("{:?}", unifier);

        if let Some(unifier) = unifier {
            assert_eq!(t0.subst(&unifier), t1.subst(&unifier));
        }
    }
}
