pub mod simple;

pub type Score = i32;

pub trait Perceptor<W> {
    type Percept;

    fn observe(&self, world: &W) -> Self::Percept;
}

pub trait Actuator<W> {
    fn apply(&self, world: &mut W);
}

pub trait Agent<W, P> {
    type A: Actuator<W>;

    fn step(&mut self, percept: P) -> (Self::A, Score);
}

pub trait World {
    fn measure(&self) -> Score;
}

pub fn simulate<W, P, A, G>(mut world: W, mut agent: G) -> Score
where
    W: World,
    A: Actuator<W>,
    G: Agent<W, P, A = A> + Perceptor<W, Percept = P>,
{
    const LIFE_SPAN: u32 = 1000;

    let mut score = 0;

    for _ in 0..LIFE_SPAN {
        score += world.measure();
        let percept = agent.observe(&world);
        let (actuator, move_score) = agent.step(percept);
        score += move_score;
        actuator.apply(&mut world);
    }

    score
}
