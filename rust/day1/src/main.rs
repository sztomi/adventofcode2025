use common::{get_lines, Result};
use std::env;
use std::path::Path;

fn parse(file: &Path) -> Result<Vec<i64>> {
  let lines = get_lines(file)?;
  Ok(
    lines
      .iter()
      .map(|line| match line.chars().next().unwrap() {
        'L' => -line[1..].parse::<i64>().unwrap(),
        'R' => line[1..].parse::<i64>().unwrap(),
        _ => panic!("Invalid char {}", line),
      })
      .collect(),
  )
}

fn solve1(data: &[i64]) -> i64 {
  data
    .iter()
    .scan(50i64, |val, &n| {
      *val = (*val + n).rem_euclid(100);
      Some(*val)
    })
    .filter(|&v| v == 0)
    .count() as i64
}

fn solve2(data: &[i64]) -> i64 {
  data
    .iter()
    .scan(50i64, |val, &n| {
      let crossings = if n > 0 {
        (*val + n).div_euclid(100) - val.div_euclid(100)
      } else if n < 0 {
        (*val - 1).div_euclid(100) - (*val + n - 1).div_euclid(100)
      } else {
        0
      };
      *val = (*val + n).rem_euclid(100);
      Some(crossings)
    })
    .sum()
}

fn main() -> Result<()> {
  let input = env::args()
    .nth(1)
    .expect("usage: day1 <input-file>");
  let nums = parse(Path::new(&input))?;
  println!("{}", solve1(&nums));
  println!("{}", solve2(&nums));
  Ok(())
}
