/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use getset::CopyGetters;
use crate::model::{Color, Piece, Square, Bitboard};
use Square::*;
use Piece::*;

#[derive(Copy, Clone, Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Board {
    pieces_layout:   [Option<Piece>; 64],
    piece_bitboards: [Bitboard; 12],
    color_bitboards: [Bitboard; 2],
}

impl Default for Board {
    fn default() -> Self {
        Board::from_pieces(&[
            (A1, WRook), (B1, WKnight), (C1, WBishop), (D1, WQueen),
            (H1, WRook), (G1, WKnight), (F1, WBishop), (E1, WKing),
            (A2, WPawn), (B2, WPawn),   (C2, WPawn),   (D2, WPawn),
            (H2, WPawn), (G2, WPawn),   (F2, WPawn),   (E2, WPawn),

            (A8, BRook), (B8, BKnight), (C8, BBishop), (D8, BQueen),
            (H8, BRook), (G8, BKnight), (F8, BBishop), (E8, BKing),
            (A7, BPawn), (B7, BPawn),   (C7, BPawn),   (D7, BPawn),
            (H7, BPawn), (G7, BPawn),   (F7, BPawn),   (E7, BPawn),
        ])
    }
}

impl Board {
    pub fn from_pieces(pieces: &[(Square, Piece)]) -> Board {
        let mut layout = [None; 64];
        for (square, piece) in pieces {
            layout[*square as usize] = Some(*piece);
        }
        Board::from_layout(layout)
    }

    pub fn from_layout(pieces_layout: [Option<Piece>; 64]) -> Board {
        let mut piece_bits = [0u64; 12];
        let mut color_bits = [0u64; 2];
        for (index, square) in pieces_layout.iter().enumerate() {
            if let Some(piece) = square {
                piece_bits[*piece as usize] |= 1 << index;
                color_bits[piece.color() as usize] |= 1 << index;
            }
        }
        Board {
            pieces_layout,
            piece_bitboards: piece_bits.map(Bitboard::from_bits),
            color_bitboards: color_bits.map(Bitboard::from_bits),
        }
    }

    pub fn place(&mut self, square: Square, piece: Option<Piece>) {
        let index = square as usize;
        if let Some(old) = self.pieces_layout[index] {
            self.piece_bitboards[old as usize].unset(square);
            self.color_bitboards[old.color() as usize].unset(square);
        }
        self.pieces_layout[index] = piece;
        if let Some(piece) = piece {
            self.piece_bitboards[piece as usize].set(square);
            self.color_bitboards[piece.color() as usize].set(square);
        }
    }

    pub fn get(&self, square: Square) -> Option<Piece> {
        self.pieces_layout[square as usize]
    }

    pub fn occupancy(&self) -> Bitboard {
        let white = self.color_bitboards[Color::White as usize];
        let black = self.color_bitboards[Color::Black as usize];
        white.merge(black)
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
    use Square::*;
    use Piece::*;

    #[test]
    fn place_and_get() {
        let mut board = Board::from_pieces(&[(E1, WKing)]);
        board.place(E1, Some(BQueen));
        board.place(D4, Some(WPawn));
        for (square, piece) in [(E1, Some(BQueen)), (D4, Some(WPawn)), (E2, None)] {
            assert_eq!(board.get(square), piece);
        }
        assert!(!board.piece_bitboards()[WKing as usize].contains(E1));
        assert!(!board.color_bitboards()[Color::White as usize].contains(E1));
    }

    #[test]
    fn occupancy() {
        let mut board = Board::from_pieces(&[(E1, WKing), (H1, BKing), (D4, WPawn)]);
        board.place(D4, None);
        assert_eq!(board.occupancy(), Bitboard::from_squares(&[E1, H1]));
    }
}
