/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color};
use super::combined::Table;

impl Board {
    pub fn middlegame_knight_heuristic(self) -> i32 {
        const VALUE: i32 = 337;
        const TABLE: Table = [
           -167, -89, -34, -49,  61, -97, -15,-107,
            -73, -41,  72,  36,  23,  62,   7, -17,
            -47,  60,  37,  65,  84, 129,  73,  44,
             -9,  17,  19,  53,  37,  69,  18,  22,
            -13,   4,  16,  13,  28,  19,  21,  -8,
            -23,  -9,  12,  10,  19,  17,  25, -16,
            -29, -53, -12,  -3,  -1,  18, -14, -19,
           -105, -21, -58, -33, -17, -28, -19, -23,
        ];
        let white = self.get_knight_mask(Color::White);
        let black = self.get_knight_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }

    pub fn endgame_knight_heuristic(self) -> i32 {
        const VALUE: i32 = 281;
        const TABLE: Table = [
            -58, -38, -13, -28, -31, -27, -63, -99,
            -25,  -8, -25,  -2,  -9, -25, -24, -52,
            -24, -20,  10,   9,  -1,  -9, -19, -41,
            -17,   3,  22,  22,  22,  11,   8, -18,
            -18,  -6,  16,  25,  16,  17,   4, -18,
            -23,  -3,  -1,  15,  10,  -3, -20, -22,
            -42, -20, -10,  -5,  -2, -20, -23, -44,
            -29, -51, -23, -15, -22, -18, -50, -64,
        ];
        let white = self.get_knight_mask(Color::White);
        let black = self.get_knight_mask(Color::Black);
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
    fn knight_heuristics() {
        let lone = Board::from_pieces(&[(E5, WKnight)]);
        assert_eq!(lone.middlegame_knight_heuristic(), 337 + 37);
        assert_eq!(lone.endgame_knight_heuristic(), 281 + 22);

        let mirrored = Board::from_pieces(&[(E5, WKnight), (E4, BKnight)]);
        assert_eq!(mirrored.middlegame_knight_heuristic(), 0);
        assert_eq!(mirrored.endgame_knight_heuristic(), 0);

        assert_eq!(Board::default().middlegame_knight_heuristic(), 0);
        assert_eq!(Board::default().endgame_knight_heuristic(), 0);
    }
}
