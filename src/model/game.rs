/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Color, Rights, Board, Piece, Square};
use anyhow::{ensure, Result};
use Piece::*;
use Square::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub en_passant_square: Option<Square>,
    pub castling_rights: Rights,
    pub halfmove_counter: u16,
    pub fullmove_counter: u16,
    pub current_turn: Color,
    pub board: Board,
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

impl Game {
    pub fn validate_game(&self) -> Result<()> {
        self.validate_kings()?;
        self.validate_pawns()?;
        self.validate_en_passant()?;
        self.validate_castling()?;
        Ok(())
    }

    fn validate_kings(&self) -> Result<()> {
        for color in [Color::White, Color::Black] {
            let kings = self.board.get_king_mask(color).bits().count_ones();
            ensure!(kings == 1, "{:?} must have exactly one king, found {}", color, kings);
        }
        let opponent = self.current_turn.opposite();
        ensure!(!self.board.is_in_check(opponent), "{:?} is in check but it is not their turn", opponent);
        Ok(())
    }

    fn validate_pawns(&self) -> Result<()> {
        let pawns = self.board.get_pawn_mask(Color::White).merge(self.board.get_pawn_mask(Color::Black));
        let back_ranks = Bitboard::from_squares(&A1.rankarray()).merge(Bitboard::from_squares(&A8.rankarray()));
        ensure!(pawns.when(back_ranks) == Bitboard::EMPTY, "pawns cannot stand on the first or last rank");
        Ok(())
    }

    fn validate_en_passant(&self) -> Result<()> {
        let Some(square) = self.en_passant_square else {
            return Ok(());
        };
        let (expected_rank, pushed_rank, pushed_pawn) = match self.current_turn {
            Color::White => (5, 4, BPawn),
            Color::Black => (2, 3, WPawn),
        };
        ensure!(square.rank() == expected_rank, "en passant square '{}' is on the wrong rank", square);
        ensure!(self.board.get(square).is_none(), "en passant square '{}' is occupied", square);
        let pushed = Square::at(square.file(), pushed_rank);
        ensure!(self.board.get(pushed) == Some(pushed_pawn), "en passant square '{}' has no pawn in front", square);
        Ok(())
    }

    fn validate_castling(&self) -> Result<()> {
        let rights = self.castling_rights;
        for (granted, king, rook) in [
            (rights.white_short, (E1, WKing), (H1, WRook)),
            (rights.white_long,  (E1, WKing), (A1, WRook)),
            (rights.black_short, (E8, BKing), (H8, BRook)),
            (rights.black_long,  (E8, BKing), (A8, BRook)),
        ] {
            let in_place = |(square, piece)| self.board.get(square) == Some(piece);
            ensure!(!granted || (in_place(king) && in_place(rook)),
                "castling right requires {:?} on {} and {:?} on {}", king.1, king.0, rook.1, rook.0);
        }
        Ok(())
    }
}

/*---------------------------------------------------------------------+
|  =========================== UNIT-TESTS ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use crate::format::FenDecode;
    use crate::model::Game;

    #[test]
    fn accepts_legal_positions() {
        for fen in [
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1",
            "4k3/8/8/8/3Pp3/8/8/4K3 b - d3 0 1",
            "8/8/8/8/8/8/8/4K2k w - - 12 40",
        ] {
            assert!(Game::fen_decode(fen).is_ok(), "rejected '{}'", fen);
        }
    }

    #[test]
    fn rejects_illegal_positions() {
        for fen in [
            "8/8/8/8/8/8/8/8 w - - 0 1",                       // no kings
            "4k3/8/8/8/8/8/8/8 w - - 0 1",                     // no white king
            "4k3/8/8/8/8/8/8/3KK3 w - - 0 1",                  // two white kings
            "4k2P/8/8/8/8/8/8/4K3 w - - 0 1",                  // pawn on rank 8
            "4k3/8/8/8/8/8/8/p3K3 w - - 0 1",                  // pawn on rank 1
            "4k3/8/8/8/8/8/8/4K2r b - - 0 1",                  // white in check, black to move
            "4k3/8/8/8/4P3/8/8/4K3 w - e3 0 1",                // ep rank wrong for side to move
            "4k3/8/8/8/8/8/8/4K3 w - d6 0 1",                  // ep without pushed pawn
            "4k3/8/3n4/3p4/8/8/8/4K3 w - d6 0 1",              // ep square occupied
            "4k3/8/8/8/8/8/8/4K3 w K - 0 1",                   // right without rook
            "r3k2r/8/8/8/8/8/8/R2K3R w Q - 0 1",               // right without king on e1
        ] {
            assert!(Game::fen_decode(fen).is_err(), "accepted '{}'", fen);
        }
    }
}
