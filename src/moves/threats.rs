/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Board, Color, Game, Piece, Square};
use crate::patterns::*;

impl Board {
    pub fn is_square_attacked_by(self, square: Square, by: Color) -> bool {
        self.is_square_attacked_by_knights(square, by) ||
        self.is_square_attacked_by_rooks(square, by)   ||
        self.is_square_attacked_by_bishops(square, by) ||
        self.is_square_attacked_by_kings(square, by)   ||
        self.is_square_attacked_by_queens(square, by)  ||
        self.is_square_attacked_by_pawns(square, by)
    }

    pub fn is_square_attacked_by_knights(self, square: Square, by: Color) -> bool {
        let piece_mask = self.get_knight_mask(by);
        let piece_squares = get_knight_squares(square);
        piece_squares.when(piece_mask) != Bitboard::EMPTY
    }

    pub fn is_square_attacked_by_rooks(self, square: Square, by: Color) -> bool {
        let occup = self.occupancy();
        let piece_mask = self.get_rook_mask(by);
        let piece_squares = get_rook_squares(square, occup);
        piece_squares.when(piece_mask) != Bitboard::EMPTY
    }

    pub fn is_square_attacked_by_bishops(self, square: Square, by: Color) -> bool {
        let occup = self.occupancy();
        let piece_mask = self.get_bishop_mask(by);
        let piece_squares = get_bishop_squares(square, occup);
        piece_squares.when(piece_mask) != Bitboard::EMPTY
    }

    pub fn is_square_attacked_by_kings(self, square: Square, by: Color) -> bool {
        let piece_mask = self.get_king_mask(by);
        let piece_squares = get_king_squares(square);
        piece_squares.when(piece_mask) != Bitboard::EMPTY
    }

    pub fn is_square_attacked_by_queens(self, square: Square, by: Color) -> bool {
        let occup = self.occupancy();
        let piece_mask = self.get_queen_mask(by);
        let piece_squares = get_queen_squares(square, occup);
        piece_squares.when(piece_mask) != Bitboard::EMPTY
    }

    pub fn is_square_attacked_by_pawns(self, square: Square, by: Color) -> bool {
        let piece_mask = self.get_pawn_mask(by);
        let piece_squares = get_pawn_attacks(square, by.opposite());
        piece_squares.when(piece_mask) != Bitboard::EMPTY
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

    #[test]
    fn sliders_and_blockers() {
        let mut board = Board::from_pieces(&[(E1, WKing), (E8, BRook), (A6, BBishop)]);
        assert!(board.is_square_attacked_by(E1, Black));
        assert!(!board.is_square_attacked_by(E1, White));
        board.place(E4, Some(WPawn));
        assert!(!board.is_square_attacked_by(E1, Black));
        assert!(board.is_square_attacked_by(F1, Black));
        board.place(C4, Some(WPawn));
        assert!(!board.is_square_attacked_by(F1, Black));
    }

    #[test]
    fn empty_squares() {
        let board = Board::from_pieces(&[(G1, WKing), (H3, BKnight)]);
        assert!(board.is_square_attacked_by(F2, Black));
        assert!(board.is_square_attacked_by(G1, Black));
        assert!(!board.is_square_attacked_by(F1, Black));
    }

    #[test]
    fn pawns_attack_diagonally_only() {
        let board = Board::from_pieces(&[(E3, BPawn), (E6, WPawn)]);
        assert!(board.is_square_attacked_by(D2, Black));
        assert!(board.is_square_attacked_by(F2, Black));
        assert!(!board.is_square_attacked_by(E2, Black));
        assert!(!board.is_square_attacked_by(D4, Black));
        assert!(board.is_square_attacked_by(D7, White));
        assert!(board.is_square_attacked_by(F7, White));
        assert!(!board.is_square_attacked_by(E7, White));
        assert!(!board.is_square_attacked_by(D5, White));
    }

    #[test]
    fn kings_and_queens() {
        let board = Board::from_pieces(&[(D4, BQueen), (H8, WKing)]);
        for square in [D8, A7, H4, G1, A1] {
            assert!(board.is_square_attacked_by(square, Black));
        }
        assert!(!board.is_square_attacked_by(E6, Black));
        assert!(board.is_square_attacked_by(G7, White));
        assert!(!board.is_square_attacked_by(F6, White));
    }
}
