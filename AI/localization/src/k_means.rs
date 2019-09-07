use rand::prelude::*;
use std::cmp::Ordering;

type Point = (f64, f64);

#[derive(Clone, Default)]
struct PointSum {
    x: f64,
    y: f64,
    cnt: usize,
}

impl PointSum {
    fn new() -> Self {
        PointSum::default()
    }

    fn add(&mut self, (x, y): Point) {
        self.x += x;
        self.y += y;
        self.cnt += 1;
    }

    fn avg(self) -> Option<Point> {
        if self.cnt == 0 {
            None
        } else {
            let cnt = self.cnt as f64;
            Some((self.x / cnt, self.y / cnt))
        }
    }
}

pub fn k_means_cluster(points: &[Point], k: usize, iter: u32) -> Vec<Point> {
    let mut rng = thread_rng();
    let mut centroids = Vec::new();
    let mut assignment = Vec::with_capacity(points.len());

    for _ in 0..points.len() {
        assignment.push(rng.gen_range(0, k));
    }

    for _ in 0..iter {
        centroids = update_centroids(k, points, &assignment);
        reassign_points(&centroids, points, &mut assignment);
    }

    centroids
}

fn update_centroids(k: usize, points: &[Point], assignment: &[usize]) -> Vec<Point> {
    let mut sums = vec![PointSum::new(); k];
    for (&p, &a) in points.iter().zip(assignment) {
        sums[a].add(p);
    }

    let mut centroids = sums
        .into_iter()
        .filter_map(|sum| sum.avg())
        .collect::<Vec<_>>();

    let mut rng = thread_rng();
    // a centroid may have no point assigned to it
    // in that case random points are padded to the vector of centroids
    while centroids.len() < k {
        centroids.push(
            *points
                .choose(&mut rng)
                .expect("update_centroids: Nonempty vector of points"),
        );
    }

    centroids
}

pub fn reassign_points(centroids: &[Point], points: &[Point], assignment: &mut [usize]) {
    for (&p, a) in points.iter().zip(assignment.iter_mut()) {
        *a = centroids
            .iter()
            .enumerate()
            .min_by(|(_, &c0), (_, &c1)| cmp_f64(distance(c0, p), distance(c1, p)))
            .map(|(i, _)| i)
            .expect("reassign_points: Nonempty vector of centroids");
    }
}

fn distance((x0, y0): Point, (x1, y1): Point) -> f64 {
    (x0 - x1).powi(2) + (y0 - y1).powi(2)
}

fn cmp_f64(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b)
        .expect("cmp_f64: non-NaN value on both side")
}

#[test]
fn k_means_test() {
    let points = vec![
        (1.0, 1.0),
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
    ];

    let centroids = k_means_cluster(&points, 4, 20);
    // when k == points.len(), centroids should converge to individual points
    for p in points {
        assert!(centroids.contains(&p));
    }
}