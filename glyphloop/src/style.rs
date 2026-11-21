#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    DarkGray,
    Rgb(u8, u8, u8),
}

impl Color {
    /// Stable byte encoding, used for screen hashing.
    pub(crate) fn code(self) -> [u8; 4] {
        match self {
            Color::Reset => [0, 0, 0, 0],
            Color::Black => [1, 0, 0, 0],
            Color::Red => [2, 0, 0, 0],
            Color::Green => [3, 0, 0, 0],
            Color::Yellow => [4, 0, 0, 0],
            Color::Blue => [5, 0, 0, 0],
            Color::Magenta => [6, 0, 0, 0],
            Color::Cyan => [7, 0, 0, 0],
            Color::White => [8, 0, 0, 0],
            Color::Gray => [9, 0, 0, 0],
            Color::DarkGray => [10, 0, 0, 0],
            Color::Rgb(r, g, b) => [11, r, g, b],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
}

impl Default for Style {
    fn default() -> Self {
        Style { fg: Color::Reset, bg: Color::Reset, bold: false }
    }
}

impl Style {
    pub fn fg(c: Color) -> Self {
        Style { fg: c, ..Style::default() }
    }
    pub fn with_bg(mut self, c: Color) -> Self {
        self.bg = c;
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::BLANK
    }
}

impl Cell {
    pub const BLANK: Cell = Cell {
        ch: ' ',
        style: Style { fg: Color::Reset, bg: Color::Reset, bold: false },
    };

    pub fn new(ch: char) -> Self {
        Cell { ch, ..Cell::BLANK }
    }
    pub fn fg(mut self, c: Color) -> Self {
        self.style.fg = c;
        self
    }
    pub fn bg(mut self, c: Color) -> Self {
        self.style.bg = c;
        self
    }
    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }
    pub fn styled(mut self, s: Style) -> Self {
        self.style = s;
        self
    }
}
