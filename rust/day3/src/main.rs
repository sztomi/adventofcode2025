use common::{get_lines, Result};
use std::env;
use std::path::Path;

fn parse(file: &Path) -> Result<Vec<Vec<u8>>> {
  let lines = get_lines(file)?;
  Ok(
    lines
      .iter()
      .map(|line| {
        line
          .chars()
          .map(|c| c.to_digit(10).unwrap() as u8)
          .collect()
      })
      .collect(),
  )
}

fn pick_max(line: &[u8], dl: usize) -> u64 {
  (0..dl)
    .fold((0usize, 0u64), |(si, num), i| {
      let ei = line.len() - (dl - i - 1);
      let f = *line[si..ei].iter().max().unwrap();
      let new_si = line[si..].iter().position(|&x| x == f).unwrap() + si + 1;
      (new_si, num * 10 + f as u64)
    })
    .1
}

fn solve1(data: &[Vec<u8>]) -> u64 {
  data.iter().map(|line| pick_max(line, 2)).sum()
}

fn solve2(data: &[Vec<u8>]) -> u64 {
  data.iter().map(|line| pick_max(line, 12)).sum()
}

fn main() -> Result<()> {
  let input = env::args().nth(1).expect("usage: day3 <input-file>");
  let nums = parse(Path::new(&input))?;
  println!("Day3 part1: {}", solve1(&nums));
  println!("Day3 part2: {}", solve2(&nums));
  Ok(())
}
