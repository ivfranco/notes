use itertools::iproduct;

use propositional::{build::*, cnf::ToCNF, Expr, Sentence};

fn main() {
    exercise_7_1();
    exercise_7_4();
    exercise_7_9();
    exercise_7_10();
    exercise_7_18();
    exercise_7_19();
    exercise_7_24();
}

struct World {
    pits: Vec<(usize, usize)>,
    wumpus: Option<(usize, usize)>,
}

impl World {
    const ROOMS: [(usize, usize); 3] = [(1, 3), (2, 2), (3, 1)];

    fn new(p0: bool, p1: bool, p2: bool, w: usize) -> Self {
        let pits: Vec<(usize, usize)> = [p0, p1, p2]
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if p { Some(Self::ROOMS[i]) } else { None })
            .collect();
        let wumpus = if w < Self::ROOMS.len() {
            Some(Self::ROOMS[w])
        } else {
            None
        };

        World { pits, wumpus }
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Pits: {:?}, ", self.pits)?;
        if let Some(w) = self.wumpus {
            write!(f, "Wumpus: {:?}", w)?;
        } else {
            write!(f, "No wumpus")?;
        }
        Ok(())
    }
}

fn exercise_7_1() {
    println!("7.1");

    let worlds: Vec<World> = iproduct!(&[true, false], &[true, false], &[true, false], 0..4)
        .map(|(p0, p1, p2, w)| World::new(*p0, *p1, *p2, w))
        .collect();

    fn kb(world: &World) -> bool {
        world.wumpus == Some((1, 3)) && world.pits == [(3, 1)]
    }

    println!("KB worlds: ");
    for (i, world) in worlds.iter().enumerate().filter(|(_, world)| kb(world)) {
        println!("  {}: {:?}", i, world);
    }

    println!("a2 worlds: ");
    for (i, world) in worlds
        .iter()
        .enumerate()
        .filter(|(_, world)| !world.pits.contains(&(2, 2)))
    {
        println!("  {}: {:?}", i, world);
    }

    println!("a3 worlds: ");
    for (i, world) in worlds
        .iter()
        .enumerate()
        .filter(|(_, world)| world.wumpus == Some((1, 3)))
    {
        println!("  {}: {:?}", i, world);
    }
}

fn exercise_7_4() {
    println!("7.4");

    let models: Vec<_> = iproduct!(&[true, false], &[true, false], &[true, false])
        .map(|(p0, p1, p2)| vec![*p0, *p1, *p2])
        .collect();

    let expr_0 = imply(var(0), var(1));
    let expr_1 = imply(expr_0.clone(), var(2));
    let s0 = Sentence::new(expr_0, 2);
    let s1 = Sentence::new(expr_1, 3);

    println!(
        "a <=> b models: {}",
        models.iter().filter(|m| s0.truth(m)).count()
    );
    println!(
        "(a <=> b) <=> c models: {}",
        models.iter().filter(|m| s1.truth(m)).count()
    );
}

fn exercise_7_9() {
    println!("7.9");

    for (i, expr) in vec![
        iff(and(var(0), var(1)), and(var(1), var(0))),
        iff(or(var(0), var(1)), or(var(1), var(0))),
        iff(
            and(and(var(0), var(1)), var(2)),
            and(var(0), and(var(1), var(2))),
        ),
        iff(
            or(or(var(0), var(1)), var(2)),
            or(var(0), or(var(1), var(2))),
        ),
        iff(imply(var(0), var(1)), imply(not(var(1)), not(var(0)))),
        iff(imply(var(0), var(1)), or(not(var(0)), var(1))),
        iff(
            iff(var(0), var(1)),
            and(imply(var(0), var(1)), imply(var(1), var(0))),
        ),
        iff(not(and(var(0), var(1))), or(not(var(0)), not(var(1)))),
        iff(not(or(var(0), var(1))), and(not(var(0)), not(var(1)))),
        iff(
            and(var(0), or(var(1), var(2))),
            or(and(var(0), var(1)), and(var(0), var(2))),
        ),
        iff(
            or(var(0), and(var(1), var(2))),
            and(or(var(0), var(1)), or(var(0), var(2))),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let sentence = Sentence::new(expr, 3);
        assert!(
            sentence.is_taotology(),
            "Exercise 7.9: {}th sentence has been miswritten",
            i + 1
        );
    }

    println!("verified all taotologies");
}

fn report(expr: &Expr) -> &'static str {
    if expr.is_taotology() {
        "valid"
    } else if not(expr.clone()).is_taotology() {
        "unsatisfiable"
    } else {
        "satisfiable"
    }
}

fn exercise_7_10() {
    println!("7.10");

    fn smoke() -> Expr {
        var(0)
    }

    fn fire() -> Expr {
        var(1)
    }

    fn heat() -> Expr {
        var(2)
    }

    fn big() -> Expr {
        var(0)
    }

    fn dumb() -> Expr {
        var(1)
    }

    for (i, expr) in [
        imply(smoke(), smoke()),
        imply(smoke(), fire()),
        imply(imply(smoke(), fire()), imply(not(smoke()), not(fire()))),
        or(smoke(), or(fire(), not(fire()))),
        iff(
            imply(and(smoke(), heat()), fire()),
            or(imply(smoke(), fire()), imply(heat(), fire())),
        ),
        imply(imply(smoke(), fire()), imply(and(smoke(), heat()), fire())),
        or(big(), or(dumb(), imply(big(), dumb()))),
    ]
    .iter()
    .enumerate()
    {
        let index: char = (b'a' + i as u8).into();
        println!("{}.  {}", index, report(expr));
    }
}

fn exercise_7_18() {
    println!("7.18");

    fn food() -> Expr {
        var(0)
    }

    fn party() -> Expr {
        var(1)
    }

    fn drinks() -> Expr {
        var(2)
    }

    let lhs = (food() >> party()) | (drinks() >> party());
    let rhs = (food() | drinks()) >> party();
    println!("{:?}", lhs.to_cnf());
    println!("{:?}", rhs.to_cnf());

    let expr = lhs >> rhs;
    println!("{}", report(&expr));

    println!("{:?}", expr.to_cnf());
    println!("{:?}", (!expr).to_cnf());
}

fn exercise_7_19() {
    println!("7.19");

    let expr = (var(0) >> var(1)) & (var(1) >> var(2)) & (var(2) >> (!var(0)));

    println!("{:?}", expr.to_dnf_expr());
}

fn exercise_7_24() {
    println!("7.24");

    fn v(c: char) -> Expr {
        var(c as usize - b'a' as usize)
    }

    let kb = (v('p') >> v('q'))
        & ((v('l') & v('m')) >> v('p'))
        & ((v('b') & v('l')) >> v('m'))
        & ((v('a') & v('p')) >> v('l'))
        & ((v('a') & v('b')) >> v('l'))
        & v('a')
        & v('b');

    let cnf = (kb & !v('q')).to_cnf();
    println!("{:?}", cnf);
    assert!(!cnf.satisfiable());
}
