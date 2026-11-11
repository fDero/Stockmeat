/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color};
use super::combined::Table;

impl Board {
    pub fn middlegame_pawn_heuristic(self) -> i32 {
        const VALUE: i32 = 82;
        const TABLE: Table = [
              0,   0,   0,   0,   0,   0,   0,   0,
             98, 134,  61,  95,  68, 126,  34, -11,
             -6,   7,  26,  31,  65,  56,  25, -20,
            -14,  13,   6,  21,  23,  12,  17, -23,
            -27,  -2,  -5,  12,  17,   6,  10, -25,
            -26,  -4,  -4, -10,   3,   3,  33, -12,
            -35,  -1, -20, -23, -15,  24,  38, -22,
              0,   0,   0,   0,   0,   0,   0,   0,
        ];
        let white = self.get_pawn_mask(Color::White);
        let black = self.get_pawn_mask(Color::Black);
        self.piece_square_score(white, black, VALUE, &TABLE)
    }

    pub fn endgame_pawn_heuristic(self) -> i32 {
        const VALUE: i32 = 94;
        const TABLE: Table = [
              0,   0,   0,   0,   0,   0,   0,   0,
            178, 173, 158, 134, 147, 132, 165, 187,
             94, 100,  85,  67,  56,  53,  82,  84,
             32,  24,  13,   5,  -2,   4,  17,  17,
             13,   9,  -3,  -7,  -7,  -8,   3,  -1,
              4,   7,  -6,   1,   0,  -5,  -1,  -8,
             13,   8,   8,  10,  13,   0,   2,  -7,
              0,   0,   0,   0,   0,   0,   0,   0,
        ];
        let white = self.get_pawn_mask(Color::White);
        let black = self.get_pawn_mask(Color::Black);
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
    fn pawn_heuristics() {
        let lone = Board::from_pieces(&[(E4, WPawn)]);
        assert_eq!(lone.middlegame_pawn_heuristic(), 82 + 17);
        assert_eq!(lone.endgame_pawn_heuristic(), 94 - 7);

        let mirrored = Board::from_pieces(&[(E4, WPawn), (E5, BPawn)]);
        assert_eq!(mirrored.middlegame_pawn_heuristic(), 0);
        assert_eq!(mirrored.endgame_pawn_heuristic(), 0);

        assert_eq!(Board::default().middlegame_pawn_heuristic(), 0);
        assert_eq!(Board::default().endgame_pawn_heuristic(), 0);
    }
}
