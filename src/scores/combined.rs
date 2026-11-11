/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Board, Piece};
use Piece::*;
pub type Table = [i32; 64];

impl Board {
    pub fn piece_square_score(self, white: Bitboard, black: Bitboard, value: i32, table: &Table) -> i32 {
        let white_score: i32 = white.iterate().map(|square| value + table[square as usize]).sum();
        let black_score: i32 = black.iterate().map(|square| value + table[square as usize ^ 56]).sum();
        white_score - black_score
    }

    pub fn endgame_combined_heuristic(self) -> i32 {
        self.endgame_knight_heuristic() +
        self.endgame_bishop_heuristic() +
        self.endgame_queen_heuristic()  +
        self.endgame_pawn_heuristic()   +
        self.endgame_king_heuristic()   +
        self.endgame_rook_heuristic()
    }

    pub fn middlegame_combined_heuristic(self) -> i32 {
        self.middlegame_knight_heuristic() +
        self.middlegame_bishop_heuristic() +
        self.middlegame_queen_heuristic()  +
        self.middlegame_pawn_heuristic()   +
        self.middlegame_king_heuristic()   +
        self.middlegame_rook_heuristic()
    }

    pub fn dynamic_combined_heuristic(self) -> i32 {
        const MAX_PHASE: i32 = 24;
        let mut phase = 0;
        const PHASE_WEIGHTS: [(Piece, Piece, i32); 4] = [
            (WKnight, BKnight, 1), (WBishop, BBishop, 1),
            (WRook,   BRook,   2), (WQueen,  BQueen,  4),
        ];
        for (wpiece, bpiece, weight) in PHASE_WEIGHTS {
            let w = self.piece_bitboards()[wpiece as usize].bits().count_ones() as i32;
            let b = self.piece_bitboards()[bpiece as usize].bits().count_ones() as i32;
            phase += weight * (w + b);
        }
        let middlegame = self.middlegame_combined_heuristic();
        let endgame = self.endgame_combined_heuristic();
        (middlegame * phase + endgame * (MAX_PHASE - phase)) / MAX_PHASE
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
    use crate::model::Square::*;

    #[test]
    fn black_reads_the_table_upside_down() {
        let table: Table = std::array::from_fn(|index| index as i32);
        let white = Bitboard::from_squares(&[E2]);
        let black = Bitboard::from_squares(&[E7]);
        let board = Board::default();
        assert_eq!(board.piece_square_score(white, Bitboard::EMPTY, 100, &table), 100 + E2 as i32);
        assert_eq!(board.piece_square_score(Bitboard::EMPTY, black, 100, &table), -(100 + E2 as i32));
        assert_eq!(board.piece_square_score(white, black, 100, &table), 0);
    }

    #[test]
    fn full_material_uses_the_middlegame_score() {
        let mut board = Board::default();
        board.place(E2, None);
        board.place(E4, Some(WPawn));
        assert_eq!(board.dynamic_combined_heuristic(), board.middlegame_combined_heuristic());
        assert_eq!(Board::default().dynamic_combined_heuristic(), 0);
    }

    #[test]
    fn kings_and_pawns_use_the_endgame_score() {
        let board = Board::from_pieces(&[(G1, WKing), (E8, BKing), (E4, WPawn), (D6, BPawn)]);
        assert_eq!(board.dynamic_combined_heuristic(), board.endgame_combined_heuristic());
    }

    #[test]
    fn partial_material_blends_both_scores() {
        let board = Board::from_pieces(&[(G1, WKing), (E8, BKing), (D1, WQueen)]);
        let (middlegame, endgame) = (board.middlegame_combined_heuristic(), board.endgame_combined_heuristic());
        assert_eq!(board.dynamic_combined_heuristic(), (middlegame * 4 + endgame * 20) / 24);
    }
}
