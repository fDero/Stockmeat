/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use getset::CopyGetters;
use crate::model::Board;
use crate::model::Square;

#[derive(Copy, Clone, Debug)]
pub struct Game {
    pub en_passant_square: Option<Square>,
    pub castling_rights: Rights,
    pub halfmove_counter: u16,
    pub fullmove_counter: u16,
    pub current_turn: Color,
    pub board: Board,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Rights {
    pub white_short: bool,
    pub white_long:  bool,
    pub black_short: bool,
    pub black_long:  bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, CopyGetters)]
pub struct Move {
    pub start: Square,
    pub landing: Square,
    pub promotion: Option<Piece>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    WPawn, WKnight, WBishop, WRook, WQueen, WKing,
    BPawn, BKnight, BBishop, BRook, BQueen, BKing,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Piece {
    pub fn color(self) -> Color {
        match self {
            Piece::WPawn | Piece::WKnight | Piece::WBishop  => Color::White,
            Piece::WRook | Piece::WQueen  | Piece::WKing    => Color::White,
            Piece::BPawn | Piece::BKnight | Piece::BBishop  => Color::Black,
            Piece::BRook | Piece::BQueen  | Piece::BKing    => Color::Black,
        }
    }
}
