/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;
use crate::patterns::*;

use Piece::*;

impl Game {
    pub fn get_legal_moves(self) -> MoveList<256> {
        let current = Some(self.current_turn);
        (0..64).map(|index| Square::from_index(index))
        .filter(|s: &Square| { self.board.get_color(*s) == current})
        .map(|start| self.get_moves_from(start))
        .fold(MoveList::default(), |mut acc, x| {
            acc.extend(x); acc
        })
    }

    fn get_moves_from(self, start: Square) -> MoveList<27> {
        let mut moves = MoveList::default();
        moves.extend(self.get_knight_moves(start));
        moves.extend(self.get_rook_moves(start));
        moves.extend(self.get_bishop_moves(start));
        moves.extend(self.get_queen_moves(start));
        moves.extend(self.get_king_moves(start));
        moves.extend(self.get_pawn_moves(start));
        moves
    }

    fn get_knight_moves(self, start: Square) -> MoveList<8> {
        let allied_mask = self.board.get_color_mask(start);
        let attack_mask = match self.board.get(start) {
            Some(WKnight)  => get_knight_squares(start),
            Some(BKnight)  => get_knight_squares(start),
            _ => Bitboard::EMPTY,
        };
        Move::from_mask(start, attack_mask.without(allied_mask))
    }

    fn get_rook_moves(self, start: Square) -> MoveList<14> {
        let allied_mask = self.board.get_color_mask(start);
        let occup_mask  = self.board.occupancy();
        let attack_mask = match self.board.get(start) {
            Some(WRook) => get_rook_squares(start, occup_mask),
            Some(BRook) => get_rook_squares(start, occup_mask),
            _ => Bitboard::EMPTY,
        };
        Move::from_mask(start, attack_mask.without(allied_mask))
    }

    fn get_bishop_moves(self, start: Square) -> MoveList<13> {
        let allied_mask = self.board.get_color_mask(start);
        let occup_mask  = self.board.occupancy();
        let attack_mask = match self.board.get(start) {
            Some(WBishop) => get_bishop_squares(start, occup_mask),
            Some(BBishop) => get_bishop_squares(start, occup_mask),
            _ => Bitboard::EMPTY,
        };
        Move::from_mask(start, attack_mask.without(allied_mask))
    }

    fn get_queen_moves(self, start: Square) -> MoveList<27> {
        let allied_mask = self.board.get_color_mask(start);
        let occup_mask  = self.board.occupancy();
        let attack_mask = match self.board.get(start) {
            Some(WQueen) => get_queen_squares(start, occup_mask),
            Some(BQueen) => get_queen_squares(start, occup_mask),
            _ => Bitboard::EMPTY,
        };
        Move::from_mask(start, attack_mask.without(allied_mask))
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
    use Square::*;

    fn game(pieces: &[(Square, Piece)]) -> Game {
        Game {
            en_passant_square: None,
            castling_rights: Rights::default(),
            halfmove_counter: 0,
            fullmove_counter: 1,
            current_turn: Color::White,
            board: Board::from_pieces(pieces),
        }
    }

    fn landings(start: Square, moves: &[Move]) -> Bitboard {
        assert!(moves.iter().all(|mv| mv.start() == start && mv.promotion().is_none()));
        let squares: Vec<Square> = moves.iter().map(|mv| mv.landing()).collect();
        Bitboard::from_squares(&squares)
    }

    #[test]
    fn knight_moves() {
        assert_eq!(game(&[(D4, WKnight)]).get_knight_moves(D4).len(), 8);
        assert_eq!(game(&[(A1, BKnight)]).get_knight_moves(A1).len(), 2);
        let game = game(&[(D4, WKnight), (C2, WPawn), (E2, BPawn)]);
        let moves = game.get_knight_moves(D4);
        assert_eq!(moves.len(), 7);
        assert_eq!(landings(D4, &moves), get_knight_squares(D4).without(Bitboard::from_squares(&[C2])));
    }

    #[test]
    fn rook_moves() {
        assert_eq!(game(&[(D4, WRook)]).get_rook_moves(D4).len(), 14);
        let game = game(&[(A1, WRook), (A3, WPawn), (C1, BKnight)]);
        let moves = game.get_rook_moves(A1);
        assert_eq!(landings(A1, &moves), Bitboard::from_squares(&[A2, B1, C1]));
    }

    #[test]
    fn bishop_moves() {
        assert_eq!(game(&[(D4, BBishop)]).get_bishop_moves(D4).len(), 13);
        let game = game(&[(C1, WBishop), (B2, WPawn), (F4, BPawn)]);
        let moves = game.get_bishop_moves(C1);
        assert_eq!(landings(C1, &moves), Bitboard::from_squares(&[D2, E3, F4]));
    }

    #[test]
    fn queen_moves() {
        assert_eq!(game(&[(D4, WQueen)]).get_queen_moves(D4).len(), 27);
        let game = game(&[(A1, BQueen), (A2, BPawn), (B2, WPawn), (C1, BKing)]);
        let moves = game.get_queen_moves(A1);
        assert_eq!(landings(A1, &moves), Bitboard::from_squares(&[B1, B2]));
    }

    #[test]
    fn wrong_piece_or_empty_square_gives_no_moves() {
        let game = game(&[(D4, WKnight), (E4, WRook)]);
        assert!(game.get_rook_moves(D4).is_empty());
        assert!(game.get_bishop_moves(D4).is_empty());
        assert!(game.get_queen_moves(D4).is_empty());
        assert!(game.get_knight_moves(E4).is_empty());
        assert!(game.get_knight_moves(H8).is_empty());
    }

    #[test]
    fn starting_position() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(Game::fen_decode(start).unwrap().get_legal_moves().len(), 20);
    }
}
