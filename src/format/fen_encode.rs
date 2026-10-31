/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;

pub trait FenEncode {
    fn fen_encode(&self) -> String;
}

impl FenEncode for Game {
    fn fen_encode(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.board.fen_encode(),
            self.current_turn.fen_encode(),
            self.castling_rights.fen_encode(),
            self.en_passant_square.fen_encode(),
            self.halfmove_counter,
            self.fullmove_counter
        )
    }
}

impl FenEncode for Board {
    fn fen_encode(&self) -> String {
        fn encode_rank(rank: &[Option<Piece>]) -> String {
            rank.iter().map(|square| square.map_or(String::from("x"),
                |piece| piece.fen_encode())).collect::<String>()
        }
        let layout = self.pieces_layout();
        let ranks  = layout.chunks(8).map(encode_rank);
        let joined = ranks.collect::<Vec<_>>().join("/");
        (1..=8).rev().fold(joined,
            |output, n| output.replace(&"x".repeat(n), &n.to_string()))
    }
}

impl FenEncode for Piece {
    fn fen_encode(&self) -> String {
        let symbol = match self {
            Piece::WPawn   => 'P', Piece::WKnight => 'N', Piece::WBishop => 'B',
            Piece::WRook   => 'R', Piece::WQueen  => 'Q', Piece::WKing   => 'K',
            Piece::BPawn   => 'p', Piece::BKnight => 'n', Piece::BBishop => 'b',
            Piece::BRook   => 'r', Piece::BQueen  => 'q', Piece::BKing   => 'k',
        };
        String::from(symbol)
    }
}

impl FenEncode for Color {
    fn fen_encode(&self) -> String {
        match self {
            Color::White => String::from("w"),
            Color::Black => String::from("b"),
        }
    }
}

impl FenEncode for Rights {
    fn fen_encode(&self) -> String {
        let chars = [
            (self.white_short, 'K'), (self.white_long, 'Q'),
            (self.black_short, 'k'), (self.black_long, 'q'),
        ];
        let filtered = chars.iter().filter(|(right, _)| *right);
        let collected: String = filtered.map(|(_, c)| c).collect();
        if collected.is_empty() { String::from("-") } else { collected }
    }
}

impl FenEncode for Option<Square> {
    fn fen_encode(&self) -> String {
        match self {
            None => String::from("-"),
            Some(square) => {
                let file = (b'a' + square.file()) as char;
                let rank = (b'1' + square.rank()) as char;
                format!("{}{}", file, rank)
            }
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

    #[test]
    fn encodes_board() {
        for (pieces, fen) in [
            (vec![(Square::E1, Piece::WKing), (Square::H1, Piece::BKing)],  "8/8/8/8/8/8/8/4K2k"),
            (vec![(Square::A8, Piece::BRook), (Square::H8, Piece::WQueen)], "r6Q/8/8/8/8/8/8/8"),
            (vec![(Square::D4, Piece::WPawn), (Square::E4, Piece::BPawn)],  "8/8/8/8/3Pp3/8/8/8"),
        ] {
            assert_eq!(Board::from_pieces(&pieces).fen_encode(), fen);
        }
    }

    #[test]
    fn encodes_rights() {
        let none = Rights { white_short: false, white_long: false, black_short: false, black_long: false };
        for (rights, fen) in [
            (Rights { white_long: true, ..none },   "Q"),
            (Rights { black_short: false, ..Default::default() },   "KQq"),
            (Rights::default(), "KQkq"),
            (none, "-"),
        ] {
            assert_eq!(rights.fen_encode(), fen);
        }
    }

    #[test]
    fn encodes_en_passant() {
        for (square, fen) in [
            (None,             "-"),    (Some(Square::E3), "e3"),
            (Some(Square::A8), "a8"),   (Some(Square::H1), "h1"),
        ] {
            assert_eq!(square.fen_encode(), fen);
        }
    }
}
