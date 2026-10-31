/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Board, Color, Move, Rights, Square};
use anyhow::{Context, Result};
use anyhow::{bail, ensure};

#[derive(Copy, Clone, Debug)]
pub struct Game {
    pub en_passant_square: Option<Square>,
    pub castling_rights: Rights,
    pub halfmove_counter: u16,
    pub fullmove_counter: u16,
    pub current_turn: Color,
    pub board: Board,
}
