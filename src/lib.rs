//! A simple `Vec` wrapper that is accessible as if it were a 2d vector.
//! Implements `Index<(x, y)>` and `IndexMut<(x, y)>`.
//!
//! The underlying `Vec` is only mutably accessible through methods of `Vec2d`.
//! If you want ownership of `Vec`, consume the `Vec2d` with `to_vec`.

use std::ops::{Index, IndexMut};
use vec2d_error::Vec2dError;
pub mod vec2d_error;
pub type Pos = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// 
pub struct Vec2d<T: Clone> {
    tiles: Vec<T>,
    width: usize
}

impl<T: Clone> Index<Pos> for Vec2d<T> {
    type Output = T;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        if x >= self.width { panic!("Tried to index with x: {}, with width: {}", x, self.width) }
        &self.tiles[y * self.width + x]
    }
}

impl<T: Clone> IndexMut<Pos> for Vec2d<T> {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        if x >= self.width { panic!("Tried to index with x: {}, with width: {}", x, self.width) }
        &mut self.tiles[y * self.width + x]
    }
}

impl<T: Clone> Vec2d<T> {
    /// Create a new `Vec2d`
    /// # Examples
    /// ```
    /// let board = board::Vec2d::new('a', 2, 3).unwrap();
    /// assert_eq!(board.tiles(), &vec!['a','a','a','a','a','a']);
    /// assert_eq!(board.width(), 2);
    /// assert_eq!(board.height(), 3);
    /// ```
    pub fn new(default: T, width: usize, height: usize) -> Result<Vec2d<T>, Vec2dError> {
        let no_tiles = width * height;
        if no_tiles == 0 {
            Err(Vec2dError::WidthOrHeightIs0{ width, height })
        } else {
            let mut tiles = Vec::with_capacity(no_tiles);
            for _ in 1..no_tiles {
                tiles.push(default.clone());
            } 
            tiles.push(default);
            Ok(Vec2d { tiles, width })
        }
    }

    /// Create a new `Vec2d` from an existing `Vec`.
    /// Moves the original vec into a `Vec2d` without copying/cloning.
    /// # Examples
    /// ```
    /// let vec = vec!['a','b','c','d'];
    /// let board = board::Vec2d::new_from_vec(vec, 2).unwrap();
    /// assert_eq!(board[(1,0)], 'b');
    /// let vec = vec!['a','b','c','d'];
    /// let board = board::Vec2d::new_from_vec(vec, 3);
    /// assert!(board.is_err());
    /// ```
    pub fn new_from_vec(input: Vec<T>, width: usize) -> Result<Vec2d<T>, Vec2dError> {
        if input.len() == 0 || width == 0 {
            Err(Vec2dError::WidthOrInputLenIs0{ input_len: input.len(), width })
        } else if input.len() % width != 0 {
            Err(Vec2dError::InputNotDivisibleByWidth{ input_len: input.len(), width })
        } else {
            Ok( Vec2d { tiles: input, width } )
        }
    }
    
    /// Apply a function to each tile in `Vec2d`
    /// # Examples
    /// ```
    /// let mut board = board::Vec2d::new('a', 2, 3).unwrap();
    /// board.for_each_tile(|tile| *tile = 'b' );
    /// assert_eq!(board.tiles(), &vec!['b','b','b','b','b','b']);
    /// ```
    pub fn for_each_tile<F: FnMut(&mut T)>(&mut self, fun: F) {
        self.tiles.iter_mut().for_each(fun);
    }

    /// Get a `&Vec<Tile>` of the `Vec2d<Tile>`
    pub fn tiles(&self) -> &Vec<T> {
        &self.tiles
    }

    /// Get the height of the `Vec2d`
    /// 
    pub fn height(&self) -> usize {
        self.tiles.len() / self.width()
    }

    /// Get the width of the `Vec2d`
    /// 
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get an `Option<&Tile>` at `(x, y)`
    /// # Examples
    /// ```
    /// let mut board = board::Vec2d::new('a', 2, 3).unwrap();
    /// assert_eq!(board.get(0, 0), Some(&'a'));
    /// assert_eq!(board.get(1, 2), Some(&'a'));
    /// assert_eq!(board.get(2, 2), None);
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width { return None }
        self.tiles.get(y * self.width + x)
    }

    /// Get an `Option<&mut Tile>` at `(x, y)`
    /// # Examples
    /// ```
    /// let mut board = board::Vec2d::new('a', 2, 3).unwrap();
    /// assert_eq!(board.get_mut(0, 0), Some(&mut 'a'));
    /// assert_eq!(board.get_mut(1, 2), Some(&mut 'a'));
    /// assert_eq!(board.get_mut(2, 2), None);
    /// ```
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width { return None }
        self.tiles.get_mut(y * self.width + x)
    }

    pub fn get_row(&self, y: usize) -> Option<&[T]> {
        if y >= self.tiles.len() / self.width { return None }
        Some(&self.tiles[y * self.width .. (y+1) * self.width])
    }

    pub fn get_row_mut(&mut self, y: usize) -> Option<&mut [T]> {
        if y >= self.tiles.len() / self.width { return None }
        Some(&mut self.tiles[y * self.width .. (y+1) * self.width])
    }

    /// Get an iterator over all `(x, y)` values: `Iterator<Item = (usize, Iterator<Item = usize>)>`
    /// # Examples
    /// ```
    /// let board = board::Vec2d::new('a', 2, 3).unwrap();
    /// for (x, row) in board.iter_xy() {
    ///     for y in row {
    ///         assert_eq!(board[(x, y)], 'a')
    ///     }
    /// }
    /// ```
    pub fn iter_xy(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = usize>)> {
        let width = self.width;
        let height = self.height();
        (0..width).map(move |i|(i, 0..height))
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.tiles
            .rchunks(self.width)
            .into_iter()
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.tiles
            .rchunks_mut(self.width)
            .into_iter()
    }

    /// Iterate over: `((x, y), &Tile)`
    /// # Examples
    /// ```
    /// let board = board::Vec2d::new(&'a', 2, 2).unwrap();
    /// assert!(
    ///     board.clone().iter_with_pos()
    ///     .all(|((x, y), tile)| board[(x, y)] == *tile )
    /// );
    /// ```
    pub fn iter_with_pos<'a>(&'a self) -> impl Iterator<Item = (Pos, &T)> {
        let width = self.width;
        self.tiles.iter().enumerate().map(move |(nr, tile)| {
            let x = nr % width;
            let y = nr / width;
            ((x, y), tile)
        })
    }

    /// Iterate over: `((x, y), &mut Tile)`
    /// # Examples
    /// ```
    /// let board = board::Vec2d::new(&'a', 2, 2).unwrap();
    /// assert!(
    ///     board.clone().iter_with_pos()
    ///     .all(|((x, y), tile)| board[(x, y)] == *tile )
    /// );
    /// ```
    pub fn iter_with_pos_mut<'a>(&'a mut self) -> impl Iterator<Item = (Pos, &'a mut T)> {
        let width = self.width;
        self.tiles.iter_mut().enumerate().map(move |(nr, tile)| {
            let x = nr % width;
            let y = nr / width;    
            ((x, y), tile)
        })
    }

    /// Consumes self to return the original `Vec`
    /// # Examples
    /// ```
    /// let vec = vec!['a','b','c','d'];
    /// let board = board::Vec2d::new_from_vec(vec.clone(), 2).unwrap();
    /// let new_vec = board.to_vec();
    /// assert_eq!(vec, new_vec);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        self.tiles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Vec2d::new('a', 2, 2).unwrap();
        assert_eq!(board,
            Vec2d{
                tiles: vec!['a', 'a', 'a', 'a'],
                width: 2
            }
        );
        let board = Vec2d::new(&'a', 0, 2);
        assert!(board.is_err())
    }

    #[test]
    fn test_get() {
        let board = Vec2d::new('a', 2, 3).unwrap();
        assert_eq!(board.get(1, 1), Some(&'a'));
        assert_eq!(board.get(0, 0), Some(&'a'));
        assert_eq!(board.get(1, 2), Some(&'a'));
        assert_eq!(board.get(2, 2), None);
        assert_eq!(board.get(1, 3), None);
    }

    #[test]
    fn test_get_mut() {
        let mut board = Vec2d::new('a', 2, 3).unwrap();
        assert_eq!(board.get_mut(1, 1), Some(&mut 'a'));
        assert_eq!(board.get_mut(0, 0), Some(&mut 'a'));
        assert_eq!(board.get_mut(1, 2), Some(&mut 'a'));
        assert_eq!(board.get_mut(2, 2), None);
        assert_eq!(board.get_mut(1, 3), None);
    }

}