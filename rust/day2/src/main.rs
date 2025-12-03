use common::{get_lines, Result};
use std::collections::HashSet;
use std::env;
use std::path::Path;

fn parse(file: &Path) -> Result<Vec<(u64, u64)>> {
    let lines = get_lines(file)?;
    let mut ranges = Vec::new();
    for pair in lines[0].split(',') {
        let parts: Vec<&str> = pair.split('-').collect();
        let l: u64 = parts[0].parse()?;
        let r: u64 = parts[1].parse()?;
        ranges.push((l, r));
    }
    Ok(ranges)
}

fn num_digits(n: u64) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

fn repints_in_range(l: u64, r: u64, only_k2: bool) -> Vec<u64> {
    let d_l = num_digits(l);
    let d_r = num_digits(r);

    let repints = (d_l..=d_r)
        .flat_map(|d| (1..d).map(move |m| (d, m)))
        .filter(|&(d, m)| d % m == 0)
        .map(|(d, m)| (m, d / m))
        .filter(|&(_, k)| k >= 2 && (!only_k2 || k == 2))
        .flat_map(|(m, k)| {
            let base = 10_u64.pow(m);
            let multiplier = (base.pow(k) - 1) / (base - 1);
            let start = 10_u64.pow(m - 1);
            (start..base)
                .map(move |b| b * multiplier)
                .take_while(move |&repint| repint <= r)
                .filter(move |&repint| repint >= l)
        });

    if only_k2 {
        repints.collect()
    } else {
        let mut seen = HashSet::new();
        repints.filter(|&repint| seen.insert(repint)).collect()
    }
}

fn solve1(data: &[(u64, u64)]) -> u64 {
    data.iter()
        .map(|&(l, r)| repints_in_range(l, r, true).iter().sum::<u64>())
        .sum()
}

fn solve2(data: &[(u64, u64)]) -> u64 {
    data.iter()
        .map(|&(l, r)| repints_in_range(l, r, false).iter().sum::<u64>())
        .sum()
}

fn main() -> Result<()> {
    let input = env::args().nth(1).expect("usage: day2 <input-file>");
    let nums = parse(Path::new(&input))?;
    println!("Day2 part1: {}", solve1(&nums));
    println!("Day2 part2: {}", solve2(&nums));
    Ok(())
}
