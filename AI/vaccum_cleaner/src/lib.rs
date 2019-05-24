
pub mod rectangle;
pub mod simple;

pub type Score = i32;

pub trait Perceptor<W> {
    type Percept;

    fn observe(&self, world: &W) -> Self::Percept;
}

pub trait Actuator<W> {
    fn apply(&self, world: &mut W);
    fn cost(&self) -> Score {
        0
    }
}

pub trait Agent<W, P> {
    type A: Actuator<W>;

    fn step(&mut self, percept: P) -> Self::A;
}

pub trait World {
    fn measure(&self) -> Score;
}

pub trait Judge<W, A>
where
    W: World,
    A: Actuator<W>,
{
    fn assert_world(&mut self, world: &W);
    fn assert_actuator(&mut self, actuator: &A);
    fn score(&self) -> Score;
}

#[derive(Default)]
pub struct MeasureJudge {
    score: Score,
}

impl<W, A> Judge<W, A> for MeasureJudge
where
    W: World,
    A: Actuator<W>,
{
    fn assert_world(&mut self, world: &W) {
        self.score += world.measure();
    }

    fn assert_actuator(&mut self, _: &A) {}

    fn score(&self) -> Score {
        self.score
    }
}

#[derive(Default)]
pub struct CostJudge {
    score: Score,
}

impl<W, A> Judge<W, A> for CostJudge
where
    W: World,
    A: Actuator<W>,
{
    fn assert_world(&mut self, world: &W) {
        self.score += world.measure();
    }

    fn assert_actuator(&mut self, actuator: &A) {
        self.score -= actuator.cost();
    }

    fn score(&self) -> Score {
        self.score
    }
}

pub fn simulate<W, P, A, G, J>(mut world: W, mut agent: G, mut judge: J) -> Score
where
    W: World,
    A: Actuator<W>,
    G: Agent<W, P, A = A> + Perceptor<W, Percept = P>,
    J: Judge<W, A>,
{
    const LIFE_SPAN: u32 = 1000;
    for _ in 0..LIFE_SPAN {
        judge.assert_world(&world);
        let percept = agent.observe(&world);
        let actuator = agent.step(percept);
        judge.assert_actuator(&actuator);
        actuator.apply(&mut world);
    }

    judge.score()
}
