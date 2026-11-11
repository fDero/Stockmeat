/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color};
use super::combined::Table;

impl Board {
    pub fn middlegame_rook_heuristic(self) -> i32 {
        const VALUE: i32 = 477;
        const TABLE: Table = [
             32,  42,  32,  51,  63,   9,  31,  43,
             27,  32,  58,  62,  80,  67,  26,  44,
             -5,  19,  26,  36,  17,  45,  61,  16,
            -24, -11,   7,  26,  24,  35,  -8, -20,
            -36, -26, -12,  -1,   9,  -7,   6, -23,
            -45, -25, -16, -17,   3,   0,  -5, -33,
            -44, -16, -20,  -9,  -1,  11,  -6, -71,
            -19, -13,   1,  17,  16,   7, -37, -26,
        ];
        let white = self.get_rook_mask(Color::White);
        let black = self.get_rook_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }

    pub fn endgame_rook_heuristic(self) -> i32 {
        const VALUE: i32 = 512;
        const TABLE: Table = [
             13,  10,  18,  15,  12,  12,   8,   5,
             11,  13,  13,  11,  -3,   3,   8,   3,
              7,   7,   7,   5,   4,  -3,  -5,  -3,
              4,   3,  13,   1,   2,   1,  -1,   2,
              3,   5,   8,   4,  -5,  -6,  -8, -11,
             -4,   0,  -5,  -1,  -7, -12,  -8, -16,
             -6,  -6,   0,   2,  -9,  -9, -11,  -3,
             -9,   2,   3,  -1,  -5, -13,   4, -20,
        ];
        let white = self.get_rook_mask(Color::White);
        let black = self.get_rook_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }
}

/*---------------------------------------------------------------------+
|  =========================== UNIT-TESTS ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use crate::model::{Board, Piece::*, Square::*};

    #[test]
    fn rook_heuristics() {
        let lone = Board::from_pieces(&[(A1, WRook)]);
        assert_eq!(lone.middlegame_rook_heuristic(), 477 - 19);
        assert_eq!(lone.endgame_rook_heuristic(), 512 - 9);

        let mirrored = Board::from_pieces(&[(A1, WRook), (A8, BRook)]);
        assert_eq!(mirrored.middlegame_rook_heuristic(), 0);
        assert_eq!(mirrored.endgame_rook_heuristic(), 0);

        assert_eq!(Board::default().middlegame_rook_heuristic(), 0);
        assert_eq!(Board::default().endgame_rook_heuristic(), 0);
    }
}
