use std::fmt;

use crate::style::Cell;

/// A finished screen: a fixed-size grid of styled cells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Grid { width, height, cells: vec![Cell::BLANK; width as usize * height as usize] }
    }

    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        (x < self.width && y < self.height)
            .then(|| &self.cells[y as usize * self.width as usize + x as usize])
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y as usize * self.width as usize + x as usize] = cell;
        }
    }

    /// The characters of one row, trailing spaces trimmed.
    pub fn row_text(&self, y: u16) -> String {
        let start = y as usize * self.width as usize;
        let row: String = self.cells[start..start + self.width as usize].iter().map(|c| c.ch).collect();
        row.trim_end().to_string()
    }

    /// Stable 64-bit FNV-1a hash of size, characters and styles.
    /// The same screen always gives the same hash, on any platform.
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |bytes: &[u8]| {
            for b in bytes {
                h ^= *b as u64;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        };
        eat(&self.width.to_le_bytes());
        eat(&self.height.to_le_bytes());
        for c in &self.cells {
            eat(&(c.ch as u32).to_le_bytes());
            eat(&c.style.fg.code());
            eat(&c.style.bg.code());
            eat(&[c.style.bold as u8]);
        }
        h
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.row_text(y))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn hash_changes_with_content_and_style() {
        let a = Grid::new(4, 2);
        let mut b = a.clone();
        assert_eq!(a.hash(), b.hash());
        b.set(1, 1, Cell::new('x'));
        assert_ne!(a.hash(), b.hash());
        let mut c = b.clone();
        c.set(1, 1, Cell::new('x').fg(Color::Red));
        assert_ne!(b.hash(), c.hash());
    }

    #[test]
    fn display_trims_rows() {
        let mut g = Grid::new(5, 2);
        g.set(0, 0, Cell::new('h'));
        g.set(1, 0, Cell::new('i'));
        assert_eq!(g.to_string(), "hi\n");
    }
}
