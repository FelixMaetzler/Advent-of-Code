use std::str::FromStr;

all_aoc::solution!(8, 2016);
enum Instruction {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

pub struct Grid<const W: usize, const H: usize> {
    array: [[bool; W]; H],
}
impl<const W: usize, const H: usize> Default for Grid<W, H> {
    fn default() -> Self {
        Self {
            array: [[false; W]; H],
        }
    }
}
impl<const W: usize, const H: usize> Grid<W, H> {
    fn rect(&mut self, x: usize, y: usize) {
        for i in 0..x {
            for j in 0..y {
                self.array[j][i] = true;
            }
        }
    }
    fn rot_col(&mut self, col_index: usize, len: usize) {
        let mut col = [false; H];
        (0..H).for_each(|i| {
            col[i] = self.array[i][col_index];
        });
        col.rotate_right(len % H);
        (0..H).for_each(|i| {
            self.array[i][col_index] = col[i];
        });
    }
    fn rot_row(&mut self, row_index: usize, len: usize) {
        let mut row = [false; W];
        (0..W).for_each(|i| {
            row[i] = self.array[row_index][i];
        });
        row.rotate_right(len % W);
        (0..W).for_each(|i| {
            self.array[row_index][i] = row[i];
        });
    }
    fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::Rect(x, y) => self.rect(x, y),
            Instruction::RotRow(x, y) => self.rot_row(x, y),
            Instruction::RotCol(x, y) => self.rot_col(x, y),
        }
    }
    fn print(&self) -> String {
        let count = W / 5;
        let mut s = String::with_capacity(W * H);
        let mut ret = String::with_capacity(count);
        for char_index in 0..count {
            for j in 0..H {
                for i in char_index * 5..5 * (char_index + 1) {
                    s.push(if self.array[j][i] { '#' } else { ' ' });
                }
                s.push('\n');
            }
            s.pop();
            ret.push(char_from_string(&s).unwrap());
            s.clear();
        }
        ret
    }
}
fn char_from_string(s: &str) -> Result<char, &str> {
    match s {
        "#### \n   # \n  #  \n #   \n#    \n#### " => Ok('Z'),
        "#### \n#    \n###  \n#    \n#    \n#    " => Ok('F'),
        "#  # \n#  # \n#### \n#  # \n#  # \n#  # " => Ok('H'),
        " ### \n#    \n#    \n ##  \n   # \n###  " => Ok('S'),
        " ##  \n#  # \n#  # \n#  # \n#  # \n ##  " => Ok('O'),
        " ##  \n#  # \n#    \n# ## \n#  # \n ### " => Ok('G'),
        "###  \n#  # \n#  # \n###  \n#    \n#    " => Ok('P'),
        x => todo!("needs to be added: {x}"),
        //x => Err(x),
    }
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_ascii_whitespace().collect::<Vec<_>>();
        if v.len() < 2 {
            return Err(());
        }
        match v[0] {
            "rect" => {
                if let Some((x, y)) = v[1].split_once('x') {
                    if let Ok(x) = x.parse() {
                        if let Ok(y) = y.parse() {
                            return Ok(Self::Rect(x, y));
                        }
                    }
                }
                Err(())
            }
            "rotate" => {
                if v.len() != 5 {
                    return Err(());
                }
                let b = if let Ok(b) = v[4].parse() {
                    b
                } else {
                    return Err(());
                };
                let a = if let Ok(a) = v[2].split('=').nth(1).unwrap_or_default().parse() {
                    a
                } else {
                    return Err(());
                };
                match v[1] {
                    "row" => Ok(Self::RotRow(a, b)),
                    "column" => Ok(Self::RotCol(a, b)),
                    _ => Err(()),
                }
            }

            _ => Err(()),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    let mut grid: Grid<50, 6> = Grid::default();
    vec.into_iter().for_each(|i| grid.execute(i));
    Some(grid.array.into_iter().flatten().filter(|b| *b).count())
}

pub fn part_two(input: &str) -> Option<String> {
    let vec = parse(input);
    let mut grid: Grid<50, 6> = Grid::default();
    vec.into_iter().for_each(|i| grid.execute(i));
    let s = grid.print();
    Some(s)
}
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(119));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("ZFHFSFOGPO".to_string()));
    }
}
