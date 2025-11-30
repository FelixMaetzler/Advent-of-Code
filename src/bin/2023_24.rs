use core::str::FromStr;

use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    i256::i256,
    position3d::Position3d,
};

all_aoc::solution!(24, 2023);

/// This function solves a * x = b where a is a matrix and x is unknown.
fn solve_lgs(a: &DenseGrid<i256>, b: Vec<i256>) -> Vec<i256> {
    debug_assert_eq!(a.width(), a.height(), "a has to be square");
    debug_assert_eq!(a.width(), b.len(), "the dimensions has to match");
    let erg = matrix_multiplication(&adjoint(a), &DenseGrid::from_iter(b.into_iter(), 1));
    debug_assert_eq!(erg.width(), 1);
    let det = determinant(a);
    debug_assert_ne!(det, i256::ZERO);
    erg.into_iter().map(|v| (v) / (det)).collect()
}
fn matrix_multiplication(a: &DenseGrid<i256>, b: &DenseGrid<i256>) -> DenseGrid<i256> {
    debug_assert_eq!(a.width(), b.height());
    let mut ret = DenseGrid::from_iter(
        core::iter::repeat_n(i256::ZERO, a.height() * b.width()),
        b.width(),
    );
    for y in 0..ret.height() {
        for x in 0..ret.width() {
            let mut erg = i256::ZERO;
            for k in 0..a.width() {
                erg = erg + a[(y, k)] * b[(k, x)];
            }
            ret[(y, x)] = erg;
        }
    }
    ret
}
fn determinant(matrix: &DenseGrid<i256>) -> i256 {
    debug_assert_eq!(matrix.width(), matrix.height(), "matrix has to be square");
    let n = matrix.width();
    // Base Case: the determiant of a 1x1 matrix is just the only element
    if n == 1 {
        return matrix[0];
    }
    let mut det = i256::ZERO;
    // You can reduce the calcualtion of the determinant of a nxn matrix to n calculations of a (n-1)*(n-1) matrix
    for i in 0..n {
        let minor = minor(matrix, (0, i));
        let sign = if i % 2 == 0 { i256::ONE } else { -i256::ONE };
        det = det + sign * *matrix.get((0, i)).unwrap() * minor;
    }
    det
}
/// calculates the minor of a given matrix at a specific index.
fn minor(matrix: &DenseGrid<i256>, index: (usize, usize)) -> i256 {
    determinant(&DenseGrid::from_iter(
        matrix
            .iter()
            .enumerate()
            .filter(|(j, _)| {
                j.to_coordinates(matrix).0 != index.0 && j.to_coordinates(matrix).1 != index.1
            })
            .map(|(_, e)| *e),
        matrix.width() - 1,
    ))
}
fn cofactor(matrix: &DenseGrid<i256>, index: (usize, usize)) -> i256 {
    let sign = if (index.0 + index.1).is_multiple_of(2) {
        i256::ONE
    } else {
        -i256::ONE
    };
    sign * minor(matrix, index)
}
fn transpose(matrix: &DenseGrid<i256>) -> DenseGrid<i256> {
    let ret = DenseGrid::from_iter_iter((0..matrix.width()).map(|x| matrix.get_col(x).copied()));
    debug_assert_eq!(matrix.width(), ret.height());
    debug_assert_eq!(matrix.height(), ret.width());
    ret
}
fn adjoint(matrix: &DenseGrid<i256>) -> DenseGrid<i256> {
    let adjoint = DenseGrid::from_iter(
        (0..matrix.len())
            .map(|i| i.to_coordinates(matrix))
            .map(|i| cofactor(matrix, i)),
        matrix.width(),
    );
    debug_assert_eq!(matrix.width(), adjoint.width());
    debug_assert_eq!(matrix.height(), adjoint.height());
    transpose(&adjoint)
}
#[derive(Clone, Copy)]
struct Hailstone {
    pos: Position3d<i256>,
    vel: Position3d<i256>,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let pos = from_str(pos);
        let vel = from_str(vel);
        Ok(Self { pos, vel })
    }
}

fn from_str(s: &str) -> Position3d<i256> {
    let mut it = s.split(',');
    let x = it.next().unwrap().trim().parse().unwrap();
    let y = it.next().unwrap().trim().parse().unwrap();
    let z = it.next().unwrap().trim().parse().unwrap();
    Position3d { x, y, z }
}

enum Intersection {
    ParallelOrIdentical,
    Past,
    Intersect(f64, f64),
}

pub fn part_one(input: &str) -> Option<u32> {
    execute(
        input,
        i256::from(200_000_000_000_000_i64),
        i256::from(400_000_000_000_000_i64),
    )
}
fn execute(input: &str, min: i256, max: i256) -> Option<u32> {
    let vec = parse(input);
    let mut ctr = 0;
    for i in 0..vec.len() - 1 {
        for j in i + 1..vec.len() {
            if solve(&vec[i], &vec[j], min, max) {
                ctr += 1;
            }
        }
    }
    Some(ctr)
}
fn solve(h1: &Hailstone, h2: &Hailstone, min: i256, max: i256) -> bool {
    let inter = calc_intersection(h1, h2);
    if let Intersection::Intersect(x, y) = inter {
        (min.to_f64().unwrap() <= x && x <= max.to_f64().unwrap())
            && (min.to_f64().unwrap() <= y && y <= max.to_f64().unwrap())
    } else {
        false
    }
}
fn calc_intersection(h1: &Hailstone, h2: &Hailstone) -> Intersection {
    let det = det_2x2(h1.vel.x, -h2.vel.x, h1.vel.y, -h2.vel.y);
    if det == i256::ZERO {
        return Intersection::ParallelOrIdentical;
    }
    let s = -h2.vel.y * (h2.pos.x - h1.pos.x) + h2.vel.x * (h2.pos.y - h1.pos.y);
    let t = -h1.vel.y * (h2.pos.x - h1.pos.x) + h1.vel.x * (h2.pos.y - h1.pos.y);
    let s = (s.to_f64().unwrap()) / (det.to_f64().unwrap());
    let t = (t.to_f64().unwrap()) / (det.to_f64().unwrap());
    if s < 0.0 || t < 0.0 {
        return Intersection::Past;
    }
    let x = s.mul_add(h1.vel.x.to_f64().unwrap(), h1.pos.x.to_f64().unwrap());
    let y = s.mul_add(h1.vel.y.to_f64().unwrap(), h1.pos.y.to_f64().unwrap());
    Intersection::Intersect(x, y)
}
fn det_2x2(a: i256, b: i256, c: i256, d: i256) -> i256 {
    a * d - b * c
}
pub fn part_two(input: &str) -> Option<i256> {
    let vec = parse(input);

    let (h1, h2, h3) = (vec[0], vec[1], vec[2]);
    let (v1, v2, v3) = (h1.vel, h2.vel, h3.vel);
    let (p1, p2, p3) = (h1.pos, h2.pos, h3.pos);

    let a = DenseGrid::from_iter_iter(
        vec![
            vec![
                -(v1.y - v2.y),
                v1.x - v2.x,
                i256::ZERO,
                p1.y - p2.y,
                -(p1.x - p2.x),
                i256::ZERO,
            ]
            .into_iter(),
            vec![
                -(v1.y - v3.y),
                v1.x - v3.x,
                i256::ZERO,
                p1.y - p3.y,
                -(p1.x - p3.x),
                i256::ZERO,
            ]
            .into_iter(),
            vec![
                i256::ZERO,
                -(v1.z - v2.z),
                v1.y - v2.y,
                i256::ZERO,
                p1.z - p2.z,
                -(p1.y - p2.y),
            ]
            .into_iter(),
            vec![
                i256::ZERO,
                -(v1.z - v3.z),
                v1.y - v3.y,
                i256::ZERO,
                p1.z - p3.z,
                -(p1.y - p3.y),
            ]
            .into_iter(),
            vec![
                -(v1.z - v2.z),
                i256::ZERO,
                v1.x - v2.x,
                p1.z - p2.z,
                i256::ZERO,
                -(p1.x - p2.x),
            ]
            .into_iter(),
            vec![
                -(v1.z - v3.z),
                i256::ZERO,
                v1.x - v3.x,
                p1.z - p3.z,
                i256::ZERO,
                -(p1.x - p3.x),
            ]
            .into_iter(),
        ]
        .into_iter(),
    );
    let b = vec![
        (p1.y * v1.x - p2.y * v2.x) - (p1.x * v1.y - p2.x * v2.y),
        (p1.y * v1.x - p3.y * v3.x) - (p1.x * v1.y - p3.x * v3.y),
        (p1.z * v1.y - p2.z * v2.y) - (p1.y * v1.z - p2.y * v2.z),
        (p1.z * v1.y - p3.z * v3.y) - (p1.y * v1.z - p3.y * v3.z),
        (p1.z * v1.x - p2.z * v2.x) - (p1.x * v1.z - p2.x * v2.z),
        (p1.z * v1.x - p3.z * v3.x) - (p1.x * v1.z - p3.x * v3.z),
    ];
    let erg = solve_lgs(&a, b);

    Some(erg[0] + erg[1] + erg[2])
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .trim()
        .lines()
        .map(|l| Hailstone::from_str(l).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = execute(
            &all_aoc::cli::read_examples_file(DAY),
            i256::from(7),
            i256::from(27),
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(11_995));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(i256::from(47)));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(i256::from(983_620_716_335_751_i64)));
    }
}
