use num_traits::identities::Zero;
use ordered_float::NotNan;

use petgraph::{algo::min_spanning_tree, data::Element, prelude::*};

use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};

use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::Index,
};

type CityID = usize;
type Pos = (Dis, Dis);
type Dis = NotNan<f64>;

fn distance((from_x, from_y): Pos, (to_x, to_y): Pos) -> Dis {
    let dx = from_x - to_x;
    let dy = from_y - to_y;

    NotNan::new((dx * dx + dy * dy).sqrt()).unwrap()
}

#[derive(Debug)]
pub struct Map {
    cities: Vec<Pos>,
}

impl Map {
    fn len(&self) -> usize {
        self.cities.len()
    }
}

impl Index<CityID> for Map {
    type Output = Pos;

    fn index(&self, index: CityID) -> &Self::Output {
        &self.cities[index]
    }
}

impl Distribution<Map> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Map {
        let mut cities = vec![];

        for _ in 0..rng.gen_range(5, 11) {
            let x = NotNan::new(random()).unwrap();
            let y = NotNan::new(random()).unwrap();
            cities.push((x, y));
        }

        Map { cities }
    }
}

#[derive(Clone, Debug)]
pub struct TSP<'a> {
    map: &'a Map,
    visited: Vec<CityID>,
}

impl<'a> TSP<'a> {
    pub fn new(start_city: CityID, map: &'a Map) -> Self {
        assert!(start_city < map.len());

        TSP {
            map,
            visited: vec![start_city],
        }
    }

    pub fn random_tour(map: &'a Map) -> Self {
        let mut visited: Vec<_> = (0..map.len()).collect();
        visited.shuffle(&mut thread_rng());

        TSP { map, visited }
    }

    fn start_pos(&self) -> Pos {
        self.map[self.visited[0]]
    }

    pub fn cost(&self) -> Dis {
        let mut cost = NotNan::zero();
        let mut sales_man = self.start_pos();

        for city_id in self.visited.iter().skip(1) {
            let city = self.map[*city_id];
            cost += distance(sales_man, city);
            sales_man = city;
        }

        cost += distance(sales_man, self.start_pos());
        cost
    }

    fn unvisited(&self) -> HashSet<CityID> {
        let mut unvisited: HashSet<_> = (0..self.map.cities.len()).collect();
        for city_id in &self.visited {
            unvisited.remove(city_id);
        }
        unvisited
    }

    pub fn heuristic(&self) -> Dis {
        let unvisited: Vec<_> = self
            .unvisited()
            .into_iter()
            .chain(self.visited.last().cloned())
            .map(|city_id| self.map[city_id])
            .collect();

        let graph: Graph<(), Dis, Undirected, usize> =
            Graph::from_edges(unvisited.iter().enumerate().flat_map(|(i, from)| {
                (&unvisited[i + 1..])
                    .iter()
                    .enumerate()
                    .map(move |(j, to)| (i, j + i + 1, distance(*from, *to)))
            }));

        min_spanning_tree(&graph).fold(NotNan::zero(), |mut sum, elem| {
            if let Element::Edge { weight, .. } = elem {
                sum += weight;
            };
            sum
        })
    }

    pub fn is_goal(&self) -> bool {
        self.visited.len() == self.map.len()
    }

    pub fn successors(&self) -> Vec<Self> {
        self.unvisited()
            .into_iter()
            .map(|city_id| {
                let mut visited = self.visited.clone();
                visited.push(city_id);

                TSP {
                    map: self.map,
                    visited,
                }
            })
            .collect()
    }

    fn successors_with_cost(&self) -> Vec<(Self, Dis)> {
        self.successors()
            .into_iter()
            .map(|succ| {
                let step_cost = succ.cost() - self.cost();
                (succ, step_cost)
            })
            .collect::<Vec<_>>()
    }

    pub fn solve(&self) -> Self {
        use pathfinding::prelude::astar;

        let (path, _) = astar(
            self,
            TSP::successors_with_cost,
            TSP::heuristic,
            TSP::is_goal,
        )
        .unwrap();

        path.last().unwrap().clone()
    }
}

impl<'a> PartialEq for TSP<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.visited == other.visited && std::ptr::eq(self.map, other.map)
    }
}

impl<'a> Eq for TSP<'a> {}

impl<'a> Hash for TSP<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.visited.hash(state);
    }
}

#[test]
fn astar_test() {
    use pathfinding::prelude::dijkstra;

    let map: Map = random();
    let start = TSP::new(0, &map);

    let (path, _) = dijkstra(&start, TSP::successors_with_cost, TSP::is_goal).unwrap();

    let dijkstra_cost = path.last().unwrap().cost();
    let epsilon: Dis = NotNan::new(1e-10).unwrap();

    assert!((dijkstra_cost - start.solve().cost()) <= epsilon);
}
