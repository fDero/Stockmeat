/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color};
use super::combined::Table;

impl Board {
    pub fn middlegame_king_heuristic(self) -> i32 {
        const VALUE: i32 = 0;
        const TABLE: Table = [
            -65,  23,  16, -15, -56, -34,   2,  13,
             29,  -1, -20,  -7,  -8,  -4, -38, -29,
             -9,  24,   2, -16, -20,   6,  22, -22,
            -17, -20, -12, -27, -30, -25, -14, -36,
            -49,  -1, -27, -39, -46, -44, -33, -51,
            -14, -14, -22, -46, -44, -30, -15, -27,
              1,   7,  -8, -64, -43, -16,   9,   8,
            -15,  36,  12, -54,   8, -28,  24,  14,
        ];
        let white = self.get_king_mask(Color::White);
        let black = self.get_king_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }

    pub fn endgame_king_heuristic(self) -> i32 {
        const VALUE: i32 = 0;
        const TABLE: Table = [
            -74, -35, -18, -18, -11,  15,   4, -17,
            -12,  17,  14,  17,  17,  38,  23,  11,
             10,  17,  23,  15,  20,  45,  44,  13,
             -8,  22,  24,  27,  26,  33,  26,   3,
            -18,  -4,  21,  24,  27,  23,   9, -11,
            -19,  -3,  11,  21,  23,  16,   7,  -9,
            -27, -11,   4,  13,  14,   4,  -5, -17,
            -53, -34, -21, -11, -28, -14, -24, -43,
        ];
        let white = self.get_king_mask(Color::White);
        let black = self.get_king_mask(Color::Black);
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
    fn king_heuristics() {
        let lone = Board::from_pieces(&[(G1, WKing)]);
        assert_eq!(lone.middlegame_king_heuristic(), 24);
        assert_eq!(lone.endgame_king_heuristic(), -24);

        let mirrored = Board::from_pieces(&[(G1, WKing), (G8, BKing)]);
        assert_eq!(mirrored.middlegame_king_heuristic(), 0);
        assert_eq!(mirrored.endgame_king_heuristic(), 0);

        assert_eq!(Board::default().middlegame_king_heuristic(), 0);
        assert_eq!(Board::default().endgame_king_heuristic(), 0);
    }
}
