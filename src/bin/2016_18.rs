all_aoc::solution!(18, 2016);
#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
    #[default]
    NonImplemented,
}
struct Field(Vec<Vec<Tile>>);
fn from_str(s: &str) -> Vec<Tile> {
    let len = s.len();
    let mut v = vec![Tile::default(); len];
    for (i, ch) in s.chars().enumerate() {
        v[i] = match ch {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!(),
        }
    }
    v
}
impl Field {
    fn new(input: &str, length: usize) -> Self {
        let first_row = from_str(input);
        let mut field = Self(vec![vec![Tile::NonImplemented; first_row.len()]; length]);
        field.0[0] = first_row;
        field
    }
    fn fill(&mut self) {
        for x in 0..self.0.len() - 1 {
            for y in 0..self.0[x].len() {
                let left = if y == 0 { Tile::Safe } else { self.0[x][y - 1] };
                let right = if y == self.0[x].len() - 1 {
                    Tile::Safe
                } else {
                    self.0[x][y + 1]
                };
                let center = self.0[x][y];
                self.0[x + 1][y] = trap(left, center, right);
            }
        }
    }
    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|v| v.iter().filter(|&&s| s == Tile::Safe).count())
            .sum()
    }
}
fn trap(left: Tile, center: Tile, right: Tile) -> Tile {
    if left == Tile::Trap && center == Tile::Trap && right != Tile::Trap {
        return Tile::Trap;
    }
    if left != Tile::Trap && center == Tile::Trap && right == Tile::Trap {
        return Tile::Trap;
    }
    if left == Tile::Trap && center != Tile::Trap && right != Tile::Trap {
        return Tile::Trap;
    }
    if left != Tile::Trap && center != Tile::Trap && right == Tile::Trap {
        return Tile::Trap;
    }
    Tile::Safe
}
pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 40)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 400_000)
}
fn solve(input: &str, length: usize) -> Option<usize> {
    let mut field = Field::new(input, length);
    field.fill();
    Some(field.count())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_989));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(19_999_894));
    }
}
