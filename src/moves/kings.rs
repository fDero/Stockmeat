/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;
use crate::patterns::*;
use crate::model::MoveList;

use Piece::*;
use Square::*;

impl Game {
    pub fn get_king_moves(self, start: Square) -> MoveList<8> {
        let allied_mask = self.board.get_color_mask(start);
        let attack_mask = match self.board.get(start) {
            Some(WKing) => get_king_squares(start),
            Some(BKing) => get_king_squares(start),
            _ => Bitboard::EMPTY,
        };
        let filtered_mask = attack_mask.without(allied_mask);
        let mut moves = Move::from_mask(start, filtered_mask);
        moves.extend(self.get_castling_moves(start));
        moves
    }

    pub fn get_castling_moves(self, start: Square) -> MoveList<2> {
        let mut moves : MoveList<2> = MoveList::default();
        let (color, candidates) = match self.board.get(start) {
            Some(WKing) => (Color::White, [(E1, G1), (E1, C1)]),
            Some(BKing) => (Color::Black, [(E8, G8), (E8, C8)]),
            _ => return moves,
        };
        let enemy = color.opposite();
        let occupancy = self.board.occupancy();
        for (king_from, king_to) in candidates {
            let has_rights = self.castling_rights.has_rights(king_from, king_to);
            let is_clear = Rights::castling_gap(king_from, king_to).when(occupancy) == Bitboard::EMPTY;
            let path = Rights::castling_path(king_from, king_to);
            let is_safe = !path.iter().any(|sq| self.board.is_square_attacked_by(*sq, enemy));
            if (start == king_from) && has_rights && is_clear && is_safe {
                let rook_transition = Rights::rook_transition(king_from, king_to).unwrap();
                moves.push(Move::new(start, king_to, None, rook_transition));
            }
        }
        moves
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
    use crate::format::FenDecode;

    const CASTLING_FEN: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";

    fn castles(fen: &str, start: Square) -> Vec<(Square, Square, (Square, Square))> {
        let game = Game::fen_decode(fen).unwrap();
        let moves = game.get_castling_moves(start);
        assert!(moves.iter().all(|mv| mv.start() == start && mv.promotion().is_none()));
        moves.iter().map(|mv| (mv.start(), mv.landing(), mv.side_effect())).collect()
    }

    #[test]
    fn both_sides_available() {
        assert_eq!(castles(CASTLING_FEN, E1), vec![(E1, G1, (H1, F1)), (E1, C1, (A1, D1))]);
        assert_eq!(castles(CASTLING_FEN, E8), vec![(E8, G8, (H8, F8)), (E8, C8, (A8, D8))]);
    }

    #[test]
    fn requires_rights() {
        assert!(castles("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1", E1).is_empty());
        assert_eq!(castles("r3k2r/8/8/8/8/8/8/R3K2R w Kq - 0 1", E1), vec![(E1, G1, (H1, F1))]);
        assert_eq!(castles("r3k2r/8/8/8/8/8/8/R3K2R w Kq - 0 1", E8), vec![(E8, C8, (A8, D8))]);
    }

    #[test]
    fn requires_empty_gap() {
        assert!(castles("r3k2r/8/8/8/8/8/8/RN2K1NR w KQkq - 0 1", E1).is_empty());
        assert_eq!(castles("r3k2r/8/8/8/8/8/8/RN2K2R w KQkq - 0 1", E1), vec![(E1, G1, (H1, F1))]);
        assert_eq!(castles("r3kb1r/8/8/8/8/8/8/R3K2R b KQkq - 0 1", E8), vec![(E8, C8, (A8, D8))]);
    }

    #[test]
    fn cannot_castle_out_of_or_through_check() {
        assert!(castles("4r1k1/8/8/8/8/8/8/R3K2R w KQ - 0 1", E1).is_empty());
        assert_eq!(castles("5rk1/8/8/8/8/8/8/R3K2R w KQ - 0 1", E1), vec![(E1, C1, (A1, D1))]);
        assert_eq!(castles("2r3k1/8/8/8/8/8/8/R3K2R w KQ - 0 1", E1), vec![(E1, G1, (H1, F1))]);
        assert_eq!(castles("r3k2r/8/8/8/8/8/8/4K1R1 b kq - 0 1", E8), vec![(E8, C8, (A8, D8))]);
    }

    #[test]
    fn attacked_rook_or_b_file_does_not_matter() {
        assert_eq!(castles("1r4k1/8/8/8/8/8/8/R3K2R w KQ - 0 1", E1).len(), 2);
        assert_eq!(castles("r5kr/8/8/8/8/8/8/R3K2R w KQ - 0 1", E1).len(), 2);
    }

    #[test]
    fn own_pieces_do_not_count_as_attackers() {
        assert_eq!(castles("r3k2r/8/8/8/8/8/3Q4/R3K2R w KQkq - 0 1", E1).len(), 2);
    }

    #[test]
    fn only_from_king_square() {
        assert!(castles(CASTLING_FEN, A1).is_empty());
        assert!(castles(CASTLING_FEN, E4).is_empty());
        assert!(castles("r3k2r/8/8/8/8/8/8/R2K3R w KQkq - 0 1", D1).is_empty());
    }

    #[test]
    fn king_moves_include_castling() {
        let game = Game::fen_decode(CASTLING_FEN).unwrap();
        let moves = game.get_king_moves(E1);
        let landings: Vec<Square> = moves.iter().map(|mv| mv.landing()).collect();
        assert_eq!(moves.len(), 7);
        for square in [D1, D2, E2, F2, F1, G1, C1] {
            assert!(landings.contains(&square));
        }
        let castle = moves.iter().find(|mv| mv.landing() == G1).unwrap();
        assert_eq!(castle.side_effect(), (H1, F1));
        let step = moves.iter().find(|mv| mv.landing() == F1).unwrap();
        assert_eq!(step.side_effect(), (E1, F1));
    }

    #[test]
    fn king_moves_exclude_allies() {
        let game = Game::fen_decode("4k3/8/8/8/8/8/3PPP2/3QK3 w - - 0 1").unwrap();
        assert_eq!(game.get_king_moves(E1).as_slice(), &[Move::from_squares(E1, F1)]);
    }

    #[test]
    fn king_moves_from_non_king_square() {
        let game = Game::fen_decode(CASTLING_FEN).unwrap();
        assert!(game.get_king_moves(A1).is_empty());
        assert!(game.get_king_moves(E4).is_empty());
    }
}
