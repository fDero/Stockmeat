/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    WPawn, WKnight, WBishop, WRook, WQueen, WKing,
    BPawn, BKnight, BBishop, BRook, BQueen, BKing,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Piece {
    pub fn color(self) -> Color {
        match self {
            Piece::WPawn | Piece::WKnight | Piece::WBishop  => Color::White,
            Piece::WRook | Piece::WQueen  | Piece::WKing    => Color::White,
            Piece::BPawn | Piece::BKnight | Piece::BBishop  => Color::Black,
            Piece::BRook | Piece::BQueen  | Piece::BKing    => Color::Black,
        }
    }
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/*---------------------------------------------------------------------+
|  =========================== UNIT-TESTS ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use Piece::*;

    #[test]
    fn color() {
        for piece in [WPawn, WKnight, WBishop, WRook, WQueen, WKing] {
            assert_eq!(piece.color(), Color::White);
        }
        for piece in [BPawn, BKnight, BBishop, BRook, BQueen, BKing] {
            assert_eq!(piece.color(), Color::Black);
        }
    }

    #[test]
    fn opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
        assert_eq!(Color::White.opposite().opposite(), Color::White);
    }
}
