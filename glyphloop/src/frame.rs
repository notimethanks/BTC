use crate::geom::IVec2;
use crate::grid::Grid;
use crate::style::{Cell, Style};

/// Drawing layers, composited bottom to top. Empty cells are transparent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Map = 0,
    Actors = 1,
    Ui = 2,
}

const LAYERS: usize = 3;

pub struct LayerBuf {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl LayerBuf {
    fn new(width: u16, height: u16) -> Self {
        LayerBuf { width, height, cells: vec![None; width as usize * height as usize] }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.cells.fill(None);
        self
    }

    pub fn fill(&mut self, cell: Cell) -> &mut Self {
        self.cells.fill(Some(cell));
        self
    }

    /// Put one cell. Out-of-bounds positions are ignored.
    pub fn put(&mut self, pos: impl Into<IVec2>, cell: Cell) -> &mut Self {
        let p = pos.into();
        if p.x >= 0 && p.y >= 0 && (p.x as u16) < self.width && (p.y as u16) < self.height {
            self.cells[p.y as usize * self.width as usize + p.x as usize] = Some(cell);
        }
        self
    }

    /// Write a string left to right, clipped at the edge.
    pub fn text(&mut self, pos: impl Into<IVec2>, s: &str, style: Style) -> &mut Self {
        let p = pos.into();
        for (i, ch) in s.chars().enumerate() {
            self.put((p.x + i as i32, p.y), Cell::new(ch).styled(style));
        }
        self
    }
}

/// What a game draws into each frame. Cleared by the engine before every `draw`.
pub struct Frame {
    width: u16,
    height: u16,
    layers: [LayerBuf; LAYERS],
}

impl Frame {
    pub(crate) fn new((width, height): (u16, u16)) -> Self {
        Frame {
            width,
            height,
            layers: std::array::from_fn(|_| LayerBuf::new(width, height)),
        }
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn layer(&mut self, layer: Layer) -> &mut LayerBuf {
        &mut self.layers[layer as usize]
    }

    pub(crate) fn clear_all(&mut self) {
        for l in &mut self.layers {
            l.clear();
        }
    }

    pub(crate) fn composite(&self) -> Grid {
        let mut grid = Grid::new(self.width, self.height);
        for layer in &self.layers {
            for (i, cell) in layer.cells.iter().enumerate() {
                if let Some(c) = cell {
                    let x = (i % self.width as usize) as u16;
                    let y = (i / self.width as usize) as u16;
                    grid.set(x, y, *c);
                }
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_layers_cover_lower_ones() {
        let mut f = Frame::new((3, 1));
        f.layer(Layer::Map).fill(Cell::new('.'));
        f.layer(Layer::Actors).put((1, 0), Cell::new('@'));
        assert_eq!(f.composite().row_text(0), ".@.");
    }
}
