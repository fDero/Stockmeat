/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color, Move, Square, Piece};
use anyhow::{Context, Result};
use anyhow::{bail, ensure};
use Square::*;
use Piece::*;

#[derive(Copy, Clone, Debug)]
pub struct Game {
    pub en_passant_square: Option<Square>,
    pub castling_rights: Rights,
    pub halfmove_counter: u16,
    pub fullmove_counter: u16,
    pub current_turn: Color,
    pub board: Board,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rights {
    pub white_short: bool,
    pub white_long:  bool,
    pub black_short: bool,
    pub black_long:  bool,
}

impl Default for Rights {
    fn default() -> Self {
        Rights {
            white_short: true,
            white_long:  true,
            black_short: true,
            black_long:  true,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            en_passant_square: None,
            castling_rights: Rights::default(),
            halfmove_counter: 0,
            fullmove_counter: 1,
            current_turn: Color::White,
            board: Board::default(),
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
    use crate::format::{FenDecode, FenEncode};

    #[test]
    fn default_is_starting_position() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(Game::default().fen_encode(), start);
        assert_eq!(Game::default().board.fen_encode(), Game::fen_decode(start).unwrap().board.fen_encode());
    }
}
