/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Board, Color, Game, Piece, Square};
use crate::patterns::*;

impl Board {
    pub fn get_color(self, square: Square) -> Option<Color> {
        let piece = self.pieces_layout()[square as usize]?;
        Some(piece.color())
    }

    pub fn get_color_mask(self, square: Square) -> Bitboard {
        match self.get_color(square) {
            Some(color) => self.color_bitboards()[color as usize],
            None => Bitboard::EMPTY,
        }
    }

    pub fn get_knight_mask(self, by: Color) -> Bitboard {
        let white_mask = self.piece_bitboards()[Piece::WKnight as usize];
        let black_mask = self.piece_bitboards()[Piece::BKnight as usize];
        self.color_bitboards()[by as usize].when(white_mask.merge(black_mask))
    }

    pub fn get_rook_mask(self, by: Color) -> Bitboard {
        let white_mask = self.piece_bitboards()[Piece::WRook as usize];
        let black_mask = self.piece_bitboards()[Piece::BRook as usize];
        self.color_bitboards()[by as usize].when(white_mask.merge(black_mask))
    }

    pub fn get_bishop_mask(self, by: Color) -> Bitboard {
        let white_mask = self.piece_bitboards()[Piece::WBishop as usize];
        let black_mask = self.piece_bitboards()[Piece::BBishop as usize];
        self.color_bitboards()[by as usize].when(white_mask.merge(black_mask))
    }

    pub fn get_king_mask(self, by: Color) -> Bitboard {
        let white_mask = self.piece_bitboards()[Piece::WKing as usize];
        let black_mask = self.piece_bitboards()[Piece::BKing as usize];
        self.color_bitboards()[by as usize].when(white_mask.merge(black_mask))
    }

    pub fn get_pawn_mask(self, by: Color) -> Bitboard {
        let white_mask = self.piece_bitboards()[Piece::WPawn as usize];
        let black_mask = self.piece_bitboards()[Piece::BPawn as usize];
        self.color_bitboards()[by as usize].when(white_mask.merge(black_mask))
    }

    pub fn get_queen_mask(self, by: Color) -> Bitboard {
        let white_mask = self.piece_bitboards()[Piece::WQueen as usize];
        let black_mask = self.piece_bitboards()[Piece::BQueen as usize];
        self.color_bitboards()[by as usize].when(white_mask.merge(black_mask))
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
    use Color::*;

    fn sample_board() -> Board {
        Board::from_pieces(&[
            (E1, WKing), (D1, WQueen), (A1, WRook), (C1, WBishop), (B1, WKnight), (E2, WPawn),
            (E8, BKing), (D8, BQueen), (H8, BRook), (F8, BBishop), (G8, BKnight), (E7, BPawn),
            (H1, WRook), (A8, BRook),
        ])
    }

    #[test]
    fn color_lookup() {
        let board = sample_board();
        assert_eq!(board.get_color(E1), Some(White));
        assert_eq!(board.get_color(E8), Some(Black));
        assert_eq!(board.get_color(E4), None);
        assert_eq!(board.get_color_mask(E1), board.color_bitboards()[White as usize]);
        assert_eq!(board.get_color_mask(E8), board.color_bitboards()[Black as usize]);
        assert_eq!(board.get_color_mask(E4), Bitboard::EMPTY);
    }

    #[test]
    fn piece_masks_by_color() {
        let board = sample_board();
        assert_eq!(board.get_king_mask(White),   Bitboard::from_squares(&[E1]));
        assert_eq!(board.get_queen_mask(Black),  Bitboard::from_squares(&[D8]));
        assert_eq!(board.get_rook_mask(White),   Bitboard::from_squares(&[A1, H1]));
        assert_eq!(board.get_rook_mask(Black),   Bitboard::from_squares(&[A8, H8]));
        assert_eq!(board.get_bishop_mask(White), Bitboard::from_squares(&[C1]));
        assert_eq!(board.get_knight_mask(Black), Bitboard::from_squares(&[G8]));
        assert_eq!(board.get_pawn_mask(White),   Bitboard::from_squares(&[E2]));
        assert_eq!(board.get_pawn_mask(Black),   Bitboard::from_squares(&[E7]));
    }

    #[test]
    fn missing_pieces_give_empty_masks() {
        let board = Board::from_pieces(&[(E1, WKing), (E8, BKing)]);
        assert_eq!(board.get_queen_mask(White), Bitboard::EMPTY);
        assert_eq!(board.get_pawn_mask(Black), Bitboard::EMPTY);
    }
}
