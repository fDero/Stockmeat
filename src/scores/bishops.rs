/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color};
use super::combined::Table;

impl Board {
    pub fn middlegame_bishop_heuristic(self) -> i32 {
        const VALUE: i32 = 365;
        const TABLE: Table = [
            -29,   4, -82, -37, -25, -42,   7,  -8,
            -26,  16, -18, -13,  30,  59,  18, -47,
            -16,  37,  43,  40,  35,  50,  37,  -2,
             -4,   5,  19,  50,  37,  37,   7,  -2,
             -6,  13,  13,  26,  34,  12,  10,   4,
              0,  15,  15,  15,  14,  27,  18,  10,
              4,  15,  16,   0,   7,  21,  33,   1,
            -33,  -3, -14, -21, -13, -12, -39, -21,
        ];
        let white = self.get_bishop_mask(Color::White);
        let black = self.get_bishop_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }

    pub fn endgame_bishop_heuristic(self) -> i32 {
        const VALUE: i32 = 297;
        const TABLE: Table = [
            -14, -21, -11,  -8,  -7,  -9, -17, -24,
             -8,  -4,   7, -12,  -3, -13,  -4, -14,
              2,  -8,   0,  -1,  -2,   6,   0,   4,
             -3,   9,  12,   9,  14,  10,   3,   2,
             -6,   3,  13,  19,   7,  10,  -3,  -9,
            -12,  -3,   8,  10,  13,   3,  -7, -15,
            -14, -18,  -7,  -1,   4,  -9, -15, -27,
            -23,  -9, -23,  -5,  -9, -16,  -5, -17,
        ];
        let white = self.get_bishop_mask(Color::White);
        let black = self.get_bishop_mask(Color::Black);
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
    fn bishop_heuristics() {
        let lone = Board::from_pieces(&[(D4, WBishop)]);
        assert_eq!(lone.middlegame_bishop_heuristic(), 365 + 26);
        assert_eq!(lone.endgame_bishop_heuristic(), 297 + 19);

        let mirrored = Board::from_pieces(&[(D4, WBishop), (D5, BBishop)]);
        assert_eq!(mirrored.middlegame_bishop_heuristic(), 0);
        assert_eq!(mirrored.endgame_bishop_heuristic(), 0);

        assert_eq!(Board::default().middlegame_bishop_heuristic(), 0);
        assert_eq!(Board::default().endgame_bishop_heuristic(), 0);
    }
}
