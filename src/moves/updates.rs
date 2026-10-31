/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;
use anyhow::{Context, Result};
use anyhow::{bail, ensure};

impl Game {
    pub fn try_apply(&self, mov: Move) -> Result<Game> {
        Ok(self.apply_unchecked(mov))
    }

    pub fn apply_unchecked(mut self, mov: Move) -> Game {
        let piece = self.board.get(mov.start()).unwrap();
        let actual_piece = mov.promotion().unwrap_or(piece);
        let is_capture = !self.board.get(mov.landing()).is_none();
        let is_pawn = piece == Piece::WPawn || piece == Piece::BPawn;
        self.fullmove_counter += (self.current_turn == Color::Black) as u16;
        self.halfmove_counter *= !(is_capture || is_pawn) as u16;
        self.halfmove_counter += !(is_capture || is_pawn) as u16;
        let rank_delta  = mov.start().rank().abs_diff(mov.landing().rank());
        let middle_rank = (mov.start().rank() + mov.landing().rank()) / 2;
        let mid_square  = Square::at(mov.start().file(), middle_rank);
        let is_pawn = piece == Piece::WPawn || piece == Piece::BPawn;
        match is_pawn && (rank_delta == 2) {
            true  => self.en_passant_square = Some(mid_square),
            false => self.en_passant_square = None,
        }
        self.current_turn = self.current_turn.opposite();
        self.castling_rights.update_rights(mov.start(), mov.landing());
        let (side_from, side_to) = mov.side_effect();
        self.board.place(side_to, self.board.get(side_from));
        self.board.place(side_from, None);
        self.board.place(mov.landing(), Some(actual_piece));
        self.board.place(mov.start(), None);
        self
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
    use Square::*;
    use Piece::*;

    const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const CASTLING_FEN: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";

    fn play(fen: &str, moves: &[(Square, Square)]) -> Game {
        let game = Game::fen_decode(fen).unwrap();
        moves.iter().fold(game, |game, &(start, landing)| {
            game.apply_unchecked(Move::from_squares(start, landing))
        })
    }

    #[test]
    fn quiet_move() {
        let game = play(STARTING_FEN, &[(G1, F3)]);
        assert_eq!(game.board.fen_encode(), "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R");
        assert_eq!(game.board.get(G1), None);
        assert_eq!(game.board.get(F3), Some(WKnight));
        assert_eq!(game.current_turn, Color::Black);
        assert_eq!(game.en_passant_square, None);
        assert_eq!(game.castling_rights.fen_encode(), "KQkq");
    }

    #[test]
    fn turn_and_fullmove_counter() {
        let game = play(STARTING_FEN, &[(G1, F3)]);
        assert_eq!((game.current_turn, game.fullmove_counter), (Color::Black, 1));
        let game = play(STARTING_FEN, &[(G1, F3), (G8, F6)]);
        assert_eq!((game.current_turn, game.fullmove_counter), (Color::White, 2));
        let game = play(STARTING_FEN, &[(G1, F3), (G8, F6), (F3, G1), (F6, G8)]);
        assert_eq!((game.current_turn, game.fullmove_counter), (Color::White, 3));
    }

    #[test]
    fn double_push_sets_en_passant_square() {
        assert_eq!(play(STARTING_FEN, &[(E2, E4)]).en_passant_square, Some(E3));
        assert_eq!(play(STARTING_FEN, &[(E2, E4), (D7, D5)]).en_passant_square, Some(D6));
        assert_eq!(play(STARTING_FEN, &[(E2, E4), (D7, D5), (G1, F3)]).en_passant_square, None);
        assert_eq!(play(STARTING_FEN, &[(E2, E3)]).en_passant_square, None);
    }

    #[test]
    fn capture() {
        let game = play("4k3/8/8/3p4/4P3/8/8/4K3 w - - 0 1", &[(E4, D5)]);
        assert_eq!(game.board.get(D5), Some(WPawn));
        assert_eq!(game.board.get(E4), None);
        assert_eq!(game.board.piece_bitboards()[BPawn as usize], Bitboard::EMPTY);
        assert_eq!(game.board.occupancy(), Bitboard::from_squares(&[E8, D5, E1]));
        assert_eq!(game.board.color_bitboards()[Color::Black as usize], Bitboard::from_squares(&[E8]));
    }

    #[test]
    fn castling_moves_the_rook() {
        for (turn, start, landing, rook, layout, rights) in [
            ("w", E1, G1, (H1, F1), "r3k2r/8/8/8/8/8/8/R4RK1", "kq"),
            ("w", E1, C1, (A1, D1), "r3k2r/8/8/8/8/8/8/2KR3R", "kq"),
            ("b", E8, G8, (H8, F8), "r4rk1/8/8/8/8/8/8/R3K2R", "KQ"),
            ("b", E8, C8, (A8, D8), "2kr3r/8/8/8/8/8/8/R3K2R", "KQ"),
        ] {
            let fen = CASTLING_FEN.replacen(" w ", &format!(" {} ", turn), 1);
            let game = Game::fen_decode(&fen).unwrap();
            let game = game.apply_unchecked(Move::new(start, landing, None, rook));
            assert_eq!(game.board.fen_encode(), layout);
            assert_eq!(game.castling_rights.fen_encode(), rights);
        }
    }

    #[test]
    fn plain_king_move_leaves_rook() {
        let game = play(CASTLING_FEN, &[(E1, G1)]);
        assert_eq!(game.board.fen_encode(), "r3k2r/8/8/8/8/8/8/R5KR");
        assert_eq!(game.castling_rights.fen_encode(), "kq");
    }

    #[test]
    fn rook_and_king_moves_revoke_rights() {
        assert_eq!(play(CASTLING_FEN, &[(H1, H4)]).castling_rights.fen_encode(), "Qkq");
        assert_eq!(play(CASTLING_FEN, &[(A1, A4)]).castling_rights.fen_encode(), "Kkq");
        assert_eq!(play(CASTLING_FEN, &[(H1, H4), (A8, A5)]).castling_rights.fen_encode(), "Qk");
        assert_eq!(play(CASTLING_FEN, &[(E1, E2)]).castling_rights.fen_encode(), "kq");
    }

    #[test]
    fn original_game_is_untouched() {
        let game = Game::fen_decode(STARTING_FEN).unwrap();
        let _ = game.apply_unchecked(Move::from_squares(E2, E4));
        assert_eq!(game.fen_encode(), STARTING_FEN);
    }

    #[test]
    fn halfmove_counter() {
        assert_eq!(play(STARTING_FEN, &[(G1, F3)]).halfmove_counter, 1);
        assert_eq!(play(STARTING_FEN, &[(G1, F3), (G8, F6)]).halfmove_counter, 2);
        assert_eq!(play("4k3/8/8/8/8/8/4P3/4K3 w - - 7 20", &[(E2, E3)]).halfmove_counter, 0);
        assert_eq!(play("4k3/8/8/3p4/4P3/8/8/4K3 w - - 7 20", &[(E4, D5)]).halfmove_counter, 0);
    }

    #[test]
    fn en_passant_capture() {
        let game = Game::fen_decode("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1").unwrap();
        let game = game.apply_unchecked(Move::new(E5, D6, None, (D5, D6)));
        assert_eq!(game.board.fen_encode(), "4k3/8/3P4/8/8/8/8/4K3");
        let game = Game::fen_decode("4k3/8/8/8/3Pp3/8/8/4K3 b - d3 0 1").unwrap();
        let game = game.apply_unchecked(Move::new(E4, D3, None, (D4, D3)));
        assert_eq!(game.board.fen_encode(), "4k3/8/8/8/8/3p4/8/4K3");
    }

    #[test]
    fn promotion() {
        let game = Game::fen_decode("8/4P3/8/8/8/8/8/k6K w - - 0 1").unwrap();
        let queen = Move::from_squares(E7, E8).promotions(Color::White)[3];
        assert_eq!(game.apply_unchecked(queen).board.get(E8), Some(WQueen));
    }

    #[test]
    fn capturing_rook_revokes_rights() {
        assert_eq!(play(CASTLING_FEN, &[(A1, A8)]).castling_rights.fen_encode(), "Kk");
    }
}
