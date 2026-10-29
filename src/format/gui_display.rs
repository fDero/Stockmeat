/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::{format::FenEncode, model::{Game, Piece, Square}};
use std::fmt::{Display, Formatter, Result};
use Square::*;
use Piece::*;

const HEADER : &str = "┌───┬───┬───┬───┬───┬───┬───┬───┐\n";
const RANKLN : &str = "│ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │\n";
const SPACER : &str = "├───┼───┼───┼───┼───┼───┼───┼───┤\n";
const FOOTER : &str = "└───┴───┴───┴───┴───┴───┴───┴───┘\n";

impl Display for Piece {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            WKing   => write!(formatter, "K"), BKing => write!(formatter, "k"),
            WQueen  => write!(formatter, "Q"), BQueen => write!(formatter, "q"),
            WRook   => write!(formatter, "R"), BRook => write!(formatter, "r"),
            WBishop => write!(formatter, "B"), BBishop => write!(formatter, "b"),
            WKnight => write!(formatter, "N"), BKnight => write!(formatter, "n"),
            WPawn   => write!(formatter, "P"), BPawn => write!(formatter, "p"),
        }
    }
}

impl Display for Game {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let fen = self.fen_encode();
        let beginnings = [A8, A7, A6, A5, A4, A3, A2, A1];
        let squares_by_rank = beginnings.map(|s| s.rankarray());
        let mut layout_fmt = String::from("");
        for rank_of_squares in squares_by_rank {
            layout_fmt.push_str(RANKLN);
            layout_fmt.push_str(SPACER);
            let iter = rank_of_squares.into_iter();
            for (square, piece) in iter.filter_map(|s| Some((s, self.board.get(s)?))) {
                let file_digit = char::from(b'1' + square.file());
                layout_fmt = layout_fmt.replace(file_digit, &piece.to_string());
            }
            layout_fmt = layout_fmt.replace(|c: char| c.is_ascii_digit(), " ");
        }
        layout_fmt.truncate(layout_fmt.len() - SPACER.len());
        write!(formatter, "{}{}{}\n{}\n", HEADER, layout_fmt, FOOTER, fen)
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
    use crate::format::FenDecode;

    #[test]
    fn draws_board() {
        let game = Game::fen_decode("8/8/8/8/8/8/8/4K2k w - - 0 1").unwrap();
        let drawn = game.to_string();
        let lines: Vec<&str> = drawn.lines().collect();
        assert_eq!(lines.len(), 19);
        assert_eq!(lines[0], HEADER.trim_end());
        assert_eq!(lines[2], SPACER.trim_end());
        assert_eq!(lines[16], FOOTER.trim_end());
        assert_eq!(lines[18], game.fen_encode());
    }
}
