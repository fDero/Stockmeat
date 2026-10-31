/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::Board;
use crate::model::Color;
use crate::model::Move;
use crate::model::MoveList;
use crate::model::Square;
use crate::model::SquareList;
use Square::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Rights {
    pub white_short: bool,
    pub white_long:  bool,
    pub black_short: bool,
    pub black_long:  bool,
}

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

    pub fn castling_path(&self, start: Square, landing: Square) -> SquareList<3> {
        match (start, landing) {
            (E1, G1) => SquareList::from([E1, F1, G1]),
            (E1, C1) => SquareList::from([E1, D1, C1]),
            (E8, G8) => SquareList::from([E8, F8, G8]),
            (E8, C8) => SquareList::from([E8, D8, C8]),
            _ => SquareList::default(),
        }
    }

    pub fn rook_landing(&self, start: Square, landing: Square) -> Option<Move> {
        match (start, landing) {
            (E1, G1) => Some(Move::from_squares(H1, F1)),
            (E1, C1) => Some(Move::from_squares(A1, D1)),
            (E8, G8) => Some(Move::from_squares(H8, F8)),
            (E8, C8) => Some(Move::from_squares(A8, D8)),
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

    const ALL: Rights = Rights { white_short: true, white_long: true, black_short: true, black_long: true };

    #[test]
    fn has_rights() {
        for (start, landing) in [(E1, G1), (E1, C1), (E8, G8), (E8, C8)] {
            assert!(ALL.has_rights(start, landing));
            assert!(!Rights::default().has_rights(start, landing));
        }
        let white_short = Rights { white_short: true, ..Default::default() };
        assert!(white_short.has_rights(E1, G1));
        assert!(!white_short.has_rights(E1, C1));
        assert!(!white_short.has_rights(E8, G8));
        for (start, landing) in [(E1, F1), (E1, H1), (E8, B8), (D1, G1)] {
            assert!(!ALL.has_rights(start, landing));
        }
    }

    #[test]
    fn rook_landing() {
        for (start, landing, expected) in [
            (E1, G1, Some(Move::from_squares(H1, F1))),
            (E1, C1, Some(Move::from_squares(A1, D1))),
            (E8, G8, Some(Move::from_squares(H8, F8))),
            (E8, C8, Some(Move::from_squares(A8, D8))),
            (E1, F1, None),   (E2, E4, None),
        ] {
            assert_eq!(ALL.rook_landing(start, landing), expected);
        }
    }

    #[test]
    fn castling_path_white_short() {
        assert_eq!(ALL.castling_path(E1, G1).as_slice(), &[E1, F1, G1]);
        assert!(ALL.castling_path(E2, E4).is_empty());
    }

    #[test]
    fn castling_path_other_sides() {
        assert_eq!(ALL.castling_path(E1, C1).as_slice(), &[E1, D1, C1]);
        assert_eq!(ALL.castling_path(E8, G8).as_slice(), &[E8, F8, G8]);
        assert_eq!(ALL.castling_path(E8, C8).as_slice(), &[E8, D8, C8]);
    }
}
