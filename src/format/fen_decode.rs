/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use anyhow::{bail, ensure};
use anyhow::{Context, Result};
use crate::model::*;
use regex::Regex;

pub trait FenDecode: Sized {
    fn fen_decode(input: &str) -> Result<Self>;
}

impl FenDecode for Game {
    fn fen_decode(input: &str) -> Result<Self> {
        let fields: Vec<&str> = input.split_whitespace().collect();
        let &[layout, turn, rights, en_passant, halfmove, fullmove] = fields.as_slice() else {
            bail!("FEN must have 6 fields, found {}", fields.len());
        };
        Ok(Game {
            board: Board::fen_decode(layout)?,
            current_turn: Color::fen_decode(turn)?,
            castling_rights: Rights::fen_decode(rights)?,
            en_passant_square: Option::<Square>::fen_decode(en_passant)?,
            halfmove_counter: halfmove.parse().context("invalid FEN counters")?,
            fullmove_counter: fullmove.parse().context("invalid FEN counters")?,
        })
    }
}

impl FenDecode for Board {
    fn fen_decode(input: &str) -> Result<Self> {
        fn decode_symbol(symbol: char) -> Result<Vec<Option<Piece>>> {
            match symbol.to_digit(10) {
                Some(empty @ 1..=8) => Ok(vec![None; empty as usize]),
                _ => Ok(vec![Some(Piece::fen_decode(&symbol.to_string())?)]),
            }
        }
        fn decode_rank(rank: &str) -> Result<Vec<Option<Piece>>> {
            let squares = rank.chars().map(decode_symbol).collect::<Result<Vec<_>>>()?.concat();
            ensure!(squares.len() == 8, "rank '{}' must have 8 squares, has {}", rank, squares.len());
            Ok(squares)
        }
        let ranks = input.split('/').map(decode_rank).collect::<Result<Vec<_>>>()?;
        ensure!(ranks.len() == 8, "board must have 8 ranks, found {}", ranks.len());
        let layout = ranks.concat().try_into().expect("8 ranks of 8 squares");
        Ok(Board::from_layout(layout))
    }
}

impl FenDecode for Piece {
    fn fen_decode(input: &str) -> Result<Self> {
        Ok(match input {
            "P" => Piece::WPawn, "N" => Piece::WKnight, "B" => Piece::WBishop,
            "R" => Piece::WRook, "Q" => Piece::WQueen,  "K" => Piece::WKing,
            "p" => Piece::BPawn, "n" => Piece::BKnight, "b" => Piece::BBishop,
            "r" => Piece::BRook, "q" => Piece::BQueen,  "k" => Piece::BKing,
            _ => bail!("invalid piece '{}'", input),
        })
    }
}

impl FenDecode for Color {
    fn fen_decode(input: &str) -> Result<Self> {
        match input {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => bail!("invalid color '{}'", input),
        }
    }
}

impl FenDecode for Rights {
    fn fen_decode(input: &str) -> Result<Self> {
        ensure!(!input.is_empty(), "castling rights must not be empty");
        ensure!(Regex::new(r"^(-|K?Q?k?q?)$")?.is_match(input), "invalid castling rights '{}'", input);
        Ok(Rights {
            white_short: input.contains("K"),
            white_long:  input.contains("Q"),
            black_short: input.contains("k"),
            black_long:  input.contains("q"),
        })
    }
}

impl FenDecode for Option<Square> {
    fn fen_decode(input: &str) -> Result<Self> {
        use Square::*;
        Ok(Some(match input {
            "-" => return Ok(None),
            "a3" => A3, "b3" => B3, "c3" => C3, "d3" => D3,
            "e3" => E3, "f3" => F3, "g3" => G3, "h3" => H3,
            "a6" => A6, "b6" => B6, "c6" => C6, "d6" => D6,
            "e6" => E6, "f6" => F6, "g6" => G6, "h6" => H6,
            _ => bail!("invalid en passant square '{}'", input),
        }))
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
    use crate::format::FenEncode;

    #[test]
    fn roundtrip() {
        for fen in [
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w Kq - 0 1",
            "8/8/8/8/8/8/8/4K2k w - - 12 40",
        ] {
            assert_eq!(Game::fen_decode(fen).unwrap().fen_encode(), fen);
        }
    }

    #[test]
    fn rejects_invalid() {
        for fen in [
            "8/8/8/8/8/8/8 w - - 0 1",      "9/8/8/8/8/8/8/8 w - - 0 1",
            "8/8/8/8/8/8/8/7X w - - 0 1",   "8/8/8/8/8/8/8/8 x - - 0 1",
            "8/8/8/8/8/8/8/8 w kQ - 0 1",   "8/8/8/8/8/8/8/8 w KK - 0 1",
            "8/8/8/8/8/8/8/8 w - e4 0 1",   "8/8/8/8/8/8/8/8 w - - 0",
        ] {
            assert!(Game::fen_decode(fen).is_err(), "accepted '{}'", fen);
        }
    }
}
