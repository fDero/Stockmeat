/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Board, Color, Game, Piece, Square, Move};
use crate::patterns::*;

impl Board {
    pub fn is_piece_pinned(self, mov: Move) -> bool {
        let (start, landing) = (mov.start(), mov.landing());
        let (correct_king, opponent) = match self.get_color(start) {
            Some(Color::White) => (Piece::WKing, Color::Black),
            Some(Color::Black) => (Piece::BKing, Color::White),
            _ => return false,
        };
        let bitboard = self.piece_bitboards()[correct_king as usize];
        let king_index = bitboard.bits().trailing_zeros() as u8;
        let king_square = match self.get(start) {
            Some(Piece::WKing) => landing,
            Some(Piece::BKing) => landing,
            _ => Square::from_index(king_index),
        };
        let mut after_move = self.clone();
        after_move.update_board(mov);
        after_move.is_square_attacked_by(king_square, opponent)
    }

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

    /// Legal moves of the side to move, as sorted UCI strings.
    fn legal_moves(fen: &str) -> Vec<String> {
        use crate::format::FenDecode;
        let game = Game::fen_decode(fen).unwrap();
        let mut moves: Vec<String> = game.get_legal_moves().iter().map(|mv| mv.to_string()).collect();
        moves.sort();
        moves
    }

    #[test]
    fn pinned_piece_moves_along_the_pin() {
        let moves = legal_moves("4r2k/8/8/8/8/8/4R3/4K3 w - - 0 1");
        assert_eq!(moves, ["e1d1", "e1d2", "e1f1", "e1f2", "e2e3", "e2e4", "e2e5", "e2e6", "e2e7", "e2e8"]);
    }

    #[test]
    fn check_must_be_blocked_or_escaped() {
        let moves = legal_moves("4r2k/8/8/8/8/8/R7/4K3 w - - 0 1");
        assert_eq!(moves, ["a2e2", "e1d1", "e1d2", "e1f1", "e1f2"]);
    }

    #[test]
    fn king_avoids_attacked_and_defended_squares() {
        let moves = legal_moves("3r3k/8/8/8/8/8/5p2/4K3 w - - 0 1");
        assert_eq!(moves, ["e1e2", "e1f1", "e1f2"]);
        let moves = legal_moves("3r3k/8/8/8/8/6b1/5p2/4K3 w - - 0 1");
        assert_eq!(moves, ["e1e2", "e1f1"]);
    }

    #[test]
    fn en_passant_discovering_check_is_illegal() {
        let moves = legal_moves("7k/8/8/K2pP2r/8/8/8/8 w - d6 0 1");
        assert!(!moves.contains(&"e5d6".to_string()));
        assert!(moves.contains(&"e5e6".to_string()));
        let moves = legal_moves("7k/8/8/3pP3/8/8/8/K7 w - d6 0 1");
        assert!(moves.contains(&"e5d6".to_string()));
    }
}
