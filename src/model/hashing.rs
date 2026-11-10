/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use getset::CopyGetters;
use crate::model::{Color, Game, Piece, Square};
use Piece::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct PositionHash {
    raw: u64,
}

impl PositionHash {
    pub fn for_game(game: &Game) -> Self {
        let pieces = game.board.piece_bitboards().map(|bitboard| bitboard.bits());
        let colors = game.board.color_bitboards().map(|bitboard| bitboard.bits());
        let both = |white: Piece, black: Piece| pieces[white as usize] | pieces[black as usize];
        let mut encoded_metadata : u64 = 0;
        let en_passant_square = game.en_passant_square.unwrap_or(Square::A1);
        encoded_metadata |= ((game.current_turn == Color::Black) as u64) << 0;
        encoded_metadata |= (game.castling_rights.white_short   as u64)  << 1;
        encoded_metadata |= (game.castling_rights.white_long    as u64)  << 2;
        encoded_metadata |= (game.castling_rights.black_short   as u64)  << 3;
        encoded_metadata |= (game.castling_rights.black_long    as u64)  << 4;
        encoded_metadata |= (en_passant_square.file()           as u64)  << 5;
        encoded_metadata |= (game.en_passant_square.is_some()   as u64)  << 8;
        let words = [
            colors[Color::White as usize], colors[Color::Black as usize],
            both(WBishop, BBishop), both(WRook, BRook), both(WQueen, BQueen),
            both(WKnight, BKnight), both(WPawn, BPawn), encoded_metadata,
        ];
        let (mut low, mut high) = (0, 0);
        for word in words {
            unsafe {
                low = std::arch::x86_64::_mm_crc32_u64(low, word);
                high = std::arch::x86_64::_mm_crc32_u64(high, word.swap_bytes());
            }
        }
        PositionHash { raw: high << 32 | low }
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
    use crate::format::{FenDecode, FenEncode, STARTING_FEN};
    use crate::model::Move;
    use std::collections::HashMap;
    use Square::*;

    fn hash(fen: &str) -> PositionHash {
        PositionHash::for_game(&Game::fen_decode(fen).unwrap())
    }

    fn play(moves: &[(Square, Square)]) -> PositionHash {
        let game = moves.iter().fold(Game::default(), |game, &(start, landing)| {
            game.apply_unchecked(Move::from_squares(start, landing))
        });
        PositionHash::for_game(&game)
    }

    #[test]
    fn default_matches_starting_fen() {
        assert_eq!(PositionHash::for_game(&Game::default()), hash(STARTING_FEN));
    }

    #[test]
    fn transpositions_share_the_hash() {
        let knights_first = play(&[(G1, F3), (G8, F6), (B1, C3), (B8, C6)]);
        let queenside_first = play(&[(B1, C3), (B8, C6), (G1, F3), (G8, F6)]);
        assert_eq!(knights_first, queenside_first);
    }

    #[test]
    fn counters_are_ignored() {
        assert_eq!(hash("4k3/8/8/8/8/8/8/R3K3 w - - 0 1"), hash("4k3/8/8/8/8/8/8/R3K3 w - - 37 60"));
    }

    #[test]
    fn position_details_change_the_hash() {
        let base = hash("r3k2r/8/8/3pP3/8/8/8/R3K2R w KQkq - 0 1");
        for fen in [
            "r3k2r/8/8/3pP3/8/8/8/R3K2R b KQkq - 0 1",      // side to move
            "r3k2r/8/8/3pP3/8/8/8/R3K2R w Kkq - 0 1",       // castling rights
            "r3k2r/8/8/3pP3/8/8/8/R3K2R w KQkq d6 0 1",     // en passant square
            "r3k2r/8/8/3pP3/8/8/8/R2K3R w kq - 0 1",        // piece placement
        ] {
            assert_ne!(hash(fen), base, "{}", fen);
        }
    }

    #[test]
    fn implied_kings_are_distinguished() {
        assert_ne!(hash("k7/8/8/8/8/8/8/3QK3 w - - 0 1"), hash("k7/8/8/8/8/8/8/3KQ3 w - - 0 1"));
    }

    #[test]
    fn en_passant_files_are_distinguished() {
        let a = hash("4k3/8/8/pP6/8/8/8/4K3 w - a6 0 1");
        let c = hash("4k3/8/8/1Pp5/8/8/8/4K3 w - c6 0 1");
        let none = hash("4k3/8/8/pP6/8/8/8/4K3 w - - 0 1");
        assert_ne!(a, none);
        assert_ne!(a, c);
    }

    #[test]
    fn no_collisions_in_shallow_tree() {
        fn walk(game: Game, depth: u32, seen: &mut HashMap<PositionHash, String>) {
            let fen = game.fen_encode();
            let key = fen.rsplitn(3, ' ').nth(2).unwrap().to_string();
            let previous = seen.entry(PositionHash::for_game(&game)).or_insert_with(|| key.clone());
            assert_eq!(*previous, key, "hash collision");
            if depth > 0 {
                for mov in game.get_legal_moves() {
                    walk(game.apply_unchecked(mov), depth - 1, seen);
                }
            }
        }
        let mut seen = HashMap::new();
        walk(Game::default(), 3, &mut seen);
        walk(Game::fen_decode("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap(), 2, &mut seen);
        assert!(seen.len() > 5_000);
    }
}
