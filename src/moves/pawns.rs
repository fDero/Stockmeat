/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;
use crate::patterns::*;
use crate::model::MoveList;

use Piece::*;
use Square::*;

impl Game {
    pub fn get_pawn_moves(self, start: Square) -> MoveList<12> {
        let allied_mask = self.board.get_color_mask(start);
        let color = self.board.get_color(start).unwrap_or(Color::White);
        let en_passant_fake_occupancy = self.en_passant_square.unwrap_or(start);
        let occup = self.board.occupancy().with(&[en_passant_fake_occupancy]);
        let attack_mask = match self.board.get(start) {
            Some(WPawn) => get_pawn_squares(start, color, occup),
            Some(BPawn) => get_pawn_squares(start, color, occup),
            _ => Bitboard::EMPTY,
        };
        let filtered_mask = attack_mask.without(allied_mask);
        let moves: MoveList<12> = Move::from_mask(start, filtered_mask);
        let victim = Square::at(en_passant_fake_occupancy.file(), start.rank());
        let unroll = (en_passant_fake_occupancy, victim);
        let enpass = Move::new(start, en_passant_fake_occupancy, None, unroll);
        moves.into_iter().flat_map(|mov| match [0u8, 7].contains(&mov.landing().rank()) {
            _ if mov.landing() == en_passant_fake_occupancy => MoveList::from_iter([enpass]),
            true => mov.promotions(color),
            false => MoveList::from_iter([mov]),
        }).collect()
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

    const CASTLING_FEN: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";

    fn pawn_moves(fen: &str, start: Square) -> Vec<Move> {
        let moves = Game::fen_decode(fen).unwrap().get_pawn_moves(start);
        assert!(moves.iter().all(|mv| mv.start() == start));
        moves.to_vec()
    }

    fn landings(moves: &[Move]) -> Bitboard {
        let squares: Vec<Square> = moves.iter().map(|mv| mv.landing()).collect();
        Bitboard::from_squares(&squares)
    }

    #[test]
    fn pawn_single_and_double_push() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let white = pawn_moves(start, E2);
        assert_eq!(white.len(), 2);
        assert_eq!(landings(&white), Bitboard::from_squares(&[E3, E4]));
        assert!(white.iter().all(|mv| mv.promotion().is_none() && mv.side_effect() == (E2, mv.landing())));
        let black = pawn_moves(start, D7);
        assert_eq!(landings(&black), Bitboard::from_squares(&[D6, D5]));
        assert_eq!(pawn_moves("4k3/8/8/8/4P3/8/8/4K3 w - - 0 1", E4), vec![Move::from_squares(E4, E5)]);
    }

    #[test]
    fn pawn_pushes_are_blocked() {
        assert!(pawn_moves("4k3/8/8/8/8/4n3/4P3/4K3 w - - 0 1", E2).is_empty());
        assert_eq!(pawn_moves("4k3/8/8/8/4n3/8/4P3/4K3 w - - 0 1", E2), vec![Move::from_squares(E2, E3)]);
        assert!(pawn_moves("4k3/4p3/4N3/8/8/8/8/4K3 b - - 0 1", E7).is_empty());
        assert!(pawn_moves("4k3/8/8/4p3/4P3/8/8/4K3 w - - 0 1", E4).is_empty());
    }

    #[test]
    fn pawn_captures_enemies_only() {
        let moves = pawn_moves("4k3/8/8/3n1N2/4P3/8/8/4K3 w - - 0 1", E4);
        assert_eq!(landings(&moves), Bitboard::from_squares(&[E5, D5]));
        let moves = pawn_moves("4k3/8/8/4p3/3P1b2/8/8/4K3 b - - 0 1", E5);
        assert_eq!(landings(&moves), Bitboard::from_squares(&[E4, D4]));
        assert_eq!(landings(&pawn_moves("4k3/8/8/8/8/1p6/P7/4K3 w - - 0 1", A2)), Bitboard::from_squares(&[A3, A4, B3]));
    }

    #[test]
    fn pawn_never_moves_backwards() {
        let moves = pawn_moves("4k3/8/8/8/4P3/3n1n2/8/4K3 w - - 0 1", E4);
        assert_eq!(moves, vec![Move::from_squares(E4, E5)]);
    }

    #[test]
    fn pawn_push_promotes() {
        let moves = pawn_moves("k7/4P3/8/8/8/8/8/K7 w - - 0 1", E7);
        let promoted: Vec<Option<Piece>> = moves.iter().map(|mv| mv.promotion()).collect();
        assert_eq!(promoted, vec![Some(WBishop), Some(WKnight), Some(WRook), Some(WQueen)]);
        assert!(moves.iter().all(|mv| mv.landing() == E8 && mv.side_effect() == (E7, E8)));
        let moves = pawn_moves("k7/8/8/8/8/8/3p4/K7 b - - 0 1", D2);
        assert_eq!(moves.len(), 4);
        assert!(moves.iter().all(|mv| mv.landing() == D1 && mv.promotion().map(Piece::color) == Some(Color::Black)));
    }

    #[test]
    fn pawn_capture_promotes() {
        let moves = pawn_moves("k2r1r2/4P3/8/8/8/8/8/K7 w - - 0 1", E7);
        assert_eq!(moves.len(), 12);
        assert_eq!(landings(&moves), Bitboard::from_squares(&[D8, E8, F8]));
        assert!(moves.iter().all(|mv| mv.promotion().is_some()));
        let moves = pawn_moves("k3r3/4P3/8/8/8/8/8/K7 w - - 0 1", E7);
        assert!(moves.is_empty());
    }

    #[test]
    fn pawn_en_passant_white() {
        let fen = "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1";
        let moves = pawn_moves(fen, E5);
        assert_eq!(landings(&moves), Bitboard::from_squares(&[E6, D6]));
        let enpass = *moves.iter().find(|mv| mv.landing() == D6).unwrap();
        assert_eq!(enpass, Move::new(E5, D6, None, (D6, D5)));
        let game = Game::fen_decode(fen).unwrap().apply_unchecked(enpass);
        assert_eq!(game.board.get(D6), Some(WPawn));
        assert_eq!(game.board.get(D5), None);
        assert_eq!(game.board.get(E5), None);
    }

    #[test]
    fn pawn_en_passant_black() {
        let fen = "4k3/8/8/8/3Pp3/8/8/4K3 b - d3 0 1";
        let moves = pawn_moves(fen, E4);
        assert_eq!(landings(&moves), Bitboard::from_squares(&[E3, D3]));
        let enpass = *moves.iter().find(|mv| mv.landing() == D3).unwrap();
        assert_eq!(enpass, Move::new(E4, D3, None, (D3, D4)));
        let game = Game::fen_decode(fen).unwrap().apply_unchecked(enpass);
        assert_eq!(game.board.get(D3), Some(BPawn));
        assert_eq!(game.board.get(D4), None);
    }

    #[test]
    fn pawn_en_passant_from_either_side() {
        let fen = "4k3/8/8/2PpP3/8/8/8/4K3 w - d6 0 1";
        assert!(pawn_moves(fen, C5).contains(&Move::new(C5, D6, None, (D6, D5))));
        assert!(pawn_moves(fen, E5).contains(&Move::new(E5, D6, None, (D6, D5))));
    }

    #[test]
    fn pawn_en_passant_requires_adjacency() {
        let moves = pawn_moves("4k3/8/8/3p3P/8/8/8/4K3 w - d6 0 1", H5);
        assert_eq!(moves, vec![Move::from_squares(H5, H6)]);
    }

    #[test]
    fn pawn_moves_from_non_pawn_square() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(pawn_moves(start, E1).is_empty());
        assert!(pawn_moves(start, G1).is_empty());
        assert!(pawn_moves(start, E4).is_empty());
        assert!(pawn_moves(CASTLING_FEN, E1).is_empty());
    }
}
