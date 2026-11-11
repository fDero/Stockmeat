/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color};
use super::combined::Table;

impl Board {
    pub fn middlegame_queen_heuristic(self) -> i32 {
        const VALUE: i32 = 1025;
        const TABLE: Table = [
            -28,   0,  29,  12,  59,  44,  43,  45,
            -24, -39,  -5,   1, -16,  57,  28,  54,
            -13, -17,   7,   8,  29,  56,  47,  57,
            -27, -27, -16, -16,  -1,  17,  -2,   1,
             -9, -26,  -9, -10,  -2,  -4,   3,  -3,
            -14,   2, -11,  -2,  -5,   2,  14,   5,
            -35,  -8,  11,   2,   8,  15,  -3,   1,
             -1, -18,  -9,  10, -15, -25, -31, -50,
        ];
        let white = self.get_queen_mask(Color::White);
        let black = self.get_queen_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }

    pub fn endgame_queen_heuristic(self) -> i32 {
        const VALUE: i32 = 936;
        const TABLE: Table = [
             -9,  22,  22,  27,  27,  19,  10,  20,
            -17,  20,  32,  41,  58,  25,  30,   0,
            -20,   6,   9,  49,  47,  35,  19,   9,
              3,  22,  24,  45,  57,  40,  57,  36,
            -18,  28,  19,  47,  31,  34,  39,  23,
            -16, -27,  15,   6,   9,  17,  10,   5,
            -22, -23, -30, -16, -16, -23, -36, -32,
            -33, -28, -22, -43,  -5, -32, -20, -41,
        ];
        let white = self.get_queen_mask(Color::White);
        let black = self.get_queen_mask(Color::Black);
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
    fn queen_heuristics() {
        let lone = Board::from_pieces(&[(D1, WQueen)]);
        assert_eq!(lone.middlegame_queen_heuristic(), 1025 + 10);
        assert_eq!(lone.endgame_queen_heuristic(), 936 - 43);

        let mirrored = Board::from_pieces(&[(D1, WQueen), (D8, BQueen)]);
        assert_eq!(mirrored.middlegame_queen_heuristic(), 0);
        assert_eq!(mirrored.endgame_queen_heuristic(), 0);

        assert_eq!(Board::default().middlegame_queen_heuristic(), 0);
        assert_eq!(Board::default().endgame_queen_heuristic(), 0);
    }
}
