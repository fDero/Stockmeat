/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Color, Game, History, MoveList};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing(MoveList<256>),
    ThreefoldRepetition,
    Checkmate(Color),
    Stalemate,
    FiftyMoves,
    NoPieces,
}

impl Game {
    pub fn status(&self, history: &History) -> GameStatus {
        let moves = self.get_legal_moves();
        let in_check = self.board.is_in_check(self.current_turn);
        let enemy = self.current_turn.opposite();
        let count = history.count(self);
        let marked = history.latest() == Some(self);
        match () {
            _ if !in_check && moves.is_empty()      => GameStatus::Stalemate,
            _ if in_check && moves.is_empty()       => GameStatus::Checkmate(enemy),
            _ if self.halfmove_counter >= 100       => GameStatus::FiftyMoves,
            _ if self.is_insufficient_material()    => GameStatus::NoPieces,
            _ if count >= 2 && marked               => GameStatus::ThreefoldRepetition,
            _ if count >= 3                         => GameStatus::ThreefoldRepetition,
            _                                       => GameStatus::Ongoing(moves),
        }
    }

    pub fn is_insufficient_material(&self) -> bool {
        let both = |mask: fn(_, Color) -> Bitboard| {
            mask(self.board, Color::White).merge(mask(self.board, Color::Black))
        };
        let heavy_or_pawns = both(|b, c| b.get_pawn_mask(c))
            .merge(both(|b, c| b.get_rook_mask(c)))
            .merge(both(|b, c| b.get_queen_mask(c)));
        let knights = both(|b, c| b.get_knight_mask(c));
        let bishops = both(|b, c| b.get_bishop_mask(c));
        let minors  = knights.merge(bishops).bits().count_ones();
        let same_colored_bishops = knights == Bitboard::EMPTY && (
            bishops.without(Bitboard::LIGHT_SQUARES) == Bitboard::EMPTY ||
            bishops.when(Bitboard::LIGHT_SQUARES) == Bitboard::EMPTY
        );
        heavy_or_pawns == Bitboard::EMPTY && (minors <= 1 || same_colored_bishops)
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

    fn status(fen: &str) -> GameStatus {
        let history = History::new();
        Game::fen_decode(fen).unwrap().status(&history)
    }

    #[test]
    fn checkmate() {
        let fools_mate = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
        assert!(matches!(status(fools_mate), GameStatus::Checkmate(_)));
        assert!(matches!(status("R5k1/5ppp/8/8/8/8/8/6K1 b - - 0 1"), GameStatus::Checkmate(_)));
    }

    #[test]
    fn stalemate() {
        assert_eq!(status("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1"), GameStatus::Stalemate);
    }

    #[test]
    fn fifty_moves() {
        assert_eq!(status("4k3/8/8/8/8/8/8/R3K3 b - - 100 80"), GameStatus::FiftyMoves);
        assert!(matches!(status("4k3/8/8/8/8/8/8/R3K3 b - - 99 80"), GameStatus::Ongoing(_)));
    }

    #[test]
    fn checkmate_beats_fifty_moves() {
        assert!(matches!(status("R5k1/5ppp/8/8/8/8/8/6K1 b - - 100 80"), GameStatus::Checkmate(_)));
    }

    #[test]
    fn insufficient_material() {
        let draw = GameStatus::NoPieces;
        for fen in [
            "4k3/8/8/8/8/8/8/4K3 w - - 0 1",      // K v K
            "4k3/8/8/8/8/8/8/4KN2 w - - 0 1",     // KN v K
            "4k3/8/8/8/8/8/8/2B1K3 w - - 0 1",    // KB v K
            "2b1k3/8/8/8/8/8/8/3BK3 w - - 0 1",   // KB v KB, both on light squares
        ] {
            assert_eq!(status(fen), draw, "{}", fen);
        }
        for fen in [
            "2b1k3/8/8/8/8/8/8/2B1K3 w - - 0 1",  // KB v KB, opposite colors
            "4k3/8/8/8/8/8/8/1NN1K3 w - - 0 1",   // KNN v K
            "4k3/8/8/8/8/8/8/2BNK3 w - - 0 1",    // KBN v K
            "4k3/8/8/8/8/8/4P3/4K3 w - - 0 1",    // KP v K
        ] {
            assert!(matches!(status(fen), GameStatus::Ongoing(_)), "{}", fen);
        }
    }
}
