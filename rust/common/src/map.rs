use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    path::Path,
};

use crate::{Dir, Point};
use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Map2D<T>
where
    T: Sized + Copy + Display + Default,
{
    pub map: Vec<Vec<T>>,
    pub w: usize,
    pub h: usize,
}

pub fn transpose_map<T: Sized + Copy>(map: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut tm = vec![];
    for i in 0..map[0].len() {
        tm.push(map.iter().cloned().map(|l| l[i]).collect_vec());
    }
    tm
}

pub fn rot90<T: Sized + Copy + Default>(map: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rm = vec![vec![T::default(); map.len()]; map[0].len()];
    for i in 0..map.len() {
        (0..map[0].len()).for_each(|j| {
            rm[j][map.len() - i - 1] = map[i][j];
        });
    }
    rm
}

pub fn rrot90<T: Sized + Copy + Default>(map: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rm = vec![vec![T::default(); map.len()]; map[0].len()];
    for i in 0..map.len() {
        (0..map[0].len()).for_each(|j| {
            rm[map[0].len() - j - 1][i] = map[i][j];
        });
    }
    rm
}

pub fn print_map(map: &[Vec<char>]) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

impl Map2D<char> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let map = crate::get_char_map(path)?;
        Ok(Self::new(map))
    }
}

impl<T: Sized + Copy + Display + Default> Map2D<T> {
    pub fn new(map: Vec<Vec<T>>) -> Self {
        let w = map[0].len();
        let h = map.len();
        Self { map, w, h }
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        (!(p.0 < 0 || p.0 >= self.h as i64 || p.1 < 0 || p.1 >= self.w as i64))
            .then(|| &self.map[p.0 as usize][p.1 as usize])
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> {
        (!(p.0 < 0 || p.0 >= self.h as i64 || p.1 < 0 || p.1 >= self.w as i64))
            .then(|| &mut self.map[p.0 as usize][p.1 as usize])
    }

    pub fn try_add(&mut self, p: &Point, d: &Dir) -> Option<T> {
        let p2 = p + d;
        (!(p2.0 < 0 || p2.0 >= self.h as i64 || p2.1 < 0 || p2.1 >= self.w as i64))
            .then(|| self.map[p2.0 as usize][p2.1 as usize])
    }

    pub fn try_add_mut(&mut self, p: &Point, d: &Dir) -> Option<&mut T> {
        let p2 = p + d;
        (!(p2.0 < 0 || p2.0 >= self.h as i64 || p2.1 < 0 || p2.1 >= self.w as i64))
            .then(|| &mut self.map[p2.0 as usize][p2.1 as usize])
    }

    pub fn set(&mut self, p: &Point, v: T) {
        self.map[p.0 as usize][p.1 as usize] = v;
    }

    pub fn coords(&self) -> Vec<Point> {
        (0..self.h)
            .cartesian_product(0..self.w)
            .map(|(i, j)| Point(i as i64, j as i64))
            .collect_vec()
    }

    pub fn around(&self, p: &Point) -> Vec<Point> {
        vec![
            Point(p.0 + 1, p.1),
            Point(p.0 - 1, p.1),
            Point(p.0, p.1 + 1),
            Point(p.0, p.1 - 1),
            Point(p.0 + 1, p.1 + 1),
            Point(p.0 + 1, p.1 - 1),
            Point(p.0 - 1, p.1 + 1),
            Point(p.0 - 1, p.1 - 1),
        ]
    }

    pub fn saround(&self, p: &Point) -> Vec<Point> {
        vec![
            Point(p.0 + 1, p.1),
            Point(p.0 - 1, p.1),
            Point(p.0, p.1 + 1),
            Point(p.0, p.1 - 1),
        ]
    }

    pub fn print(&self) {
        self.map.iter().for_each(|l| {
            l.iter().for_each(|c| print!("{}", c));
            println!();
        });
    }

    pub fn on_edge(&self, p: &Point) -> bool {
        p.0 == 0 || p.0 == self.h as i64 - 1 || p.1 == 0 || p.1 == self.w as i64 - 1
    }
}

impl<T: Sized + Copy + Display + Default> Index<Point> for Map2D<T> {
    type Output = T;

    fn index(&self, p: Point) -> &Self::Output {
        &self.map[p.0 as usize][p.1 as usize]
    }
}

impl<T: Sized + Copy + Display + Default> IndexMut<Point> for Map2D<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        &mut self.map[p.0 as usize][p.1 as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rrot90() {
        let map = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let rmap = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
        assert_eq!(rrot90(&map), rmap);
    }
}
