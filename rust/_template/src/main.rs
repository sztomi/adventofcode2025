use common::{get_lines, Result};
use std::env;
use std::path::Path;

fn parse(file: &Path) -> Result<Vec<i64>> {
  let lines = get_lines(file)?;
  Ok(

  )
}

fn solve1(data: &[i64]) -> i64 {
  
}

fn solve2(data: &[i64]) -> i64 {
  
}

fn main() -> Result<()> {
  let input = env::args()
    .nth(1)
    .expect("usage: day1 <input-file>");
  let nums = parse(Path::new(&input))?;
  println!("Day@DAY@ part1: {}", solve1(&nums));
  println!("Day@DAY@ part2: {}", solve2(&nums));
  Ok(())
}
