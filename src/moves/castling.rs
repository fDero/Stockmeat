/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Rights, Square, Piece, SquareList, Move, Bitboard};
use Square::*;

impl Rights {
    pub fn has_rights(&self, start: Square, landing: Square) -> bool {
        match (start, landing) {
            (E1, G1) => self.white_short,
            (E1, C1) => self.white_long,
            (E8, G8) => self.black_short,
            (E8, C8) => self.black_long,
            _ => false,
        }
    }

    pub fn update_rights(&mut self, start: Square, landing: Square) {
        for square in [start, landing] {
            match square {
                E1 => (self.white_short, self.white_long) = (false, false),
                E8 => (self.black_short, self.black_long) = (false, false),
                H1 => self.white_short = false,
                A1 => self.white_long  = false,
                H8 => self.black_short = false,
                A8 => self.black_long  = false,
                _ => {}
            }
        }
    }

    pub fn castling_path(start: Square, landing: Square) -> SquareList<3> {
        match (start, landing) {
            (E1, G1) => SquareList::from([E1, F1, G1]),
            (E1, C1) => SquareList::from([E1, D1, C1]),
            (E8, G8) => SquareList::from([E8, F8, G8]),
            (E8, C8) => SquareList::from([E8, D8, C8]),
            _ => SquareList::default(),
        }
    }

    pub fn castling_gap(start: Square, landing: Square) -> Bitboard {
        match (start, landing) {
            (E1, G1) => Bitboard::from_squares(&[F1, G1]),
            (E1, C1) => Bitboard::from_squares(&[D1, C1, B1]),
            (E8, G8) => Bitboard::from_squares(&[F8, G8]),
            (E8, C8) => Bitboard::from_squares(&[D8, C8, B8]),
            _ => Bitboard::EMPTY,
        }
    }

    pub fn rook_transition(start: Square, landing: Square) -> Option<(Square, Square)> {
        match (start, landing) {
            (E1, G1) => Some((H1, F1)),
            (E1, C1) => Some((A1, D1)),
            (E8, G8) => Some((H8, F8)),
            (E8, C8) => Some((A8, D8)),
            _ => None,
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

    const ALL:  Rights = Rights { white_short: true,  white_long: true,  black_short: true,  black_long: true  };
    const NONE: Rights = Rights { white_short: false, white_long: false, black_short: false, black_long: false };

    #[test]
    fn default_grants_all_rights() {
        assert_eq!(Rights::default(), ALL);
    }

    #[test]
    fn has_rights() {
        for (start, landing) in [(E1, G1), (E1, C1), (E8, G8), (E8, C8)] {
            assert!(ALL.has_rights(start, landing));
            assert!(!NONE.has_rights(start, landing));
        }
        let white_short = Rights { white_short: true, ..NONE };
        assert!(white_short.has_rights(E1, G1));
        assert!(!white_short.has_rights(E1, C1));
        assert!(!white_short.has_rights(E8, G8));
        for (start, landing) in [(E1, F1), (E1, H1), (E8, B8), (D1, G1)] {
            assert!(!ALL.has_rights(start, landing));
        }
    }

    #[test]
    fn castling_path_white_short() {
        assert_eq!(Rights::castling_path(E1, G1).as_slice(), &[E1, F1, G1]);
        assert!(Rights::castling_path(E2, E4).is_empty());
    }

    #[test]
    fn castling_path_other_sides() {
        assert_eq!(Rights::castling_path(E1, C1).as_slice(), &[E1, D1, C1]);
        assert_eq!(Rights::castling_path(E8, G8).as_slice(), &[E8, F8, G8]);
        assert_eq!(Rights::castling_path(E8, C8).as_slice(), &[E8, D8, C8]);
    }

    #[test]
    fn castling_gap() {
        assert_eq!(Rights::castling_gap(E1, G1), Bitboard::from_squares(&[F1, G1]));
        assert_eq!(Rights::castling_gap(E1, C1), Bitboard::from_squares(&[B1, C1, D1]));
        assert_eq!(Rights::castling_gap(E8, G8), Bitboard::from_squares(&[F8, G8]));
        assert_eq!(Rights::castling_gap(E8, C8), Bitboard::from_squares(&[B8, C8, D8]));
        assert_eq!(Rights::castling_gap(E2, E4), Bitboard::EMPTY);
    }

    #[test]
    fn rook_transition() {
        assert_eq!(Rights::rook_transition(E1, G1), Some((H1, F1)));
        assert_eq!(Rights::rook_transition(E1, C1), Some((A1, D1)));
        assert_eq!(Rights::rook_transition(E8, G8), Some((H8, F8)));
        assert_eq!(Rights::rook_transition(E8, C8), Some((A8, D8)));
        assert_eq!(Rights::rook_transition(E1, F1), None);
        assert_eq!(Rights::rook_transition(E8, G1), None);
    }

    #[test]
    fn king_moves_revoke_both_sides() {
        let mut rights = ALL;
        rights.update_rights(E1, E2);
        assert_eq!(rights, Rights { white_short: false, white_long: false, ..ALL });
        rights.update_rights(E8, D8);
        assert_eq!(rights, NONE);
    }

    #[test]
    fn rook_squares_revoke_one_side() {
        for (square, expected) in [
            (H1, Rights { white_short: false, ..ALL }),
            (A1, Rights { white_long:  false, ..ALL }),
            (H8, Rights { black_short: false, ..ALL }),
            (A8, Rights { black_long:  false, ..ALL }),
        ] {
            let mut from = ALL;
            from.update_rights(square, D4);
            assert_eq!(from, expected);
            let mut onto = ALL;
            onto.update_rights(D4, square);
            assert_eq!(onto, expected);
        }
    }

    #[test]
    fn unrelated_squares_keep_rights() {
        let mut rights = ALL;
        rights.update_rights(B1, C3);
        rights.update_rights(G8, F6);
        assert_eq!(rights, ALL);
    }
}
