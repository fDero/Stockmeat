/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::fmt::{Display, Formatter, Result};
use std::time::Duration;
use crate::search::Score;
use crate::model::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub best_variation:  MoveList<300>,
    pub search_duration: Duration,
    pub minimum_depth:   u32,
    pub maximum_depth:   u32,
    pub updated_score:   Score,
    pub nodes_counter:   u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UciOutgoing {
    Identify, UciOk, ReadyOk,
    BestMove(MoveList<2>),
    Info(SearchResult),
}

impl Display for Score {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match (self.mate_in(), self.centipawns()) {
            (Some(plies), _) => write!(formatter, "mate {}", (plies + 1).div_euclid(2)),
            (None, centipawns) => write!(formatter, "cp {}", centipawns.unwrap_or_default()),
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let millis = self.search_duration.as_millis();
        let micros = self.search_duration.as_micros().max(1);
        let nps = self.nodes_counter as u128 * 1_000_000 / micros;
        let formatted = self.best_variation.iter().map(|mov| format!(" {mov}"));
        let moves: String = formatted.collect();
        let pv = match moves.len() {
            0  => String::new(),
            _ => format!(" pv{moves}"),
        };
        write!(formatter, "info depth {} seldepth {} score {} nodes {} nps {} time {}{}",
            self.minimum_depth, self.maximum_depth, self.updated_score,
            self.nodes_counter, nps, millis, pv,
        )
    }
}

impl Display for UciOutgoing {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let identity = "id name Stockmeat\nid author fDero";
        match self {
            UciOutgoing::UciOk => write!(formatter, "uciok"),
            UciOutgoing::ReadyOk => write!(formatter, "readyok"),
            UciOutgoing::Identify => write!(formatter, "{}", identity),
            UciOutgoing::Info(result) => write!(formatter, "{result}"),
            UciOutgoing::BestMove(list) => match list.len() {
                0 => write!(formatter, "bestmove 0000"),
                1 => write!(formatter, "bestmove {}", list[0]),
                _ => write!(formatter, "bestmove {} ponder {}", list[0], list[1]),
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
    use Square::*;

    fn result(score: Score, pv: &[Move], millis: u64) -> SearchResult {
        SearchResult {
            best_variation: pv.iter().copied().collect(),
            search_duration: Duration::from_millis(millis),
            minimum_depth: 3,
            maximum_depth: 7,
            updated_score: score,
            nodes_counter: 5000,
        }
    }

    #[test]
    fn handshake_messages() {
        assert_eq!(UciOutgoing::Identify.to_string(), "id name Stockmeat\nid author fDero");
        assert_eq!(UciOutgoing::UciOk.to_string(), "uciok");
        assert_eq!(UciOutgoing::ReadyOk.to_string(), "readyok");
    }

    #[test]
    fn bestmove_variants() {
        let e2e4 = Move::from_squares(E2, E4);
        let e7e5 = Move::from_squares(E7, E5);
        let bestmove = |moves: &[Move]| UciOutgoing::BestMove(moves.iter().copied().collect()).to_string();
        assert_eq!(bestmove(&[e2e4]), "bestmove e2e4");
        assert_eq!(bestmove(&[e2e4, e7e5]), "bestmove e2e4 ponder e7e5");
        assert_eq!(bestmove(&[]), "bestmove 0000");
    }

    #[test]
    fn info_with_pv() {
        let pv = [Move::from_squares(E2, E4), Move::from_squares(E7, E5)];
        let info = UciOutgoing::Info(result(Score::from_centipawns(-35), &pv, 50));
        assert_eq!(info.to_string(), "info depth 3 seldepth 7 score cp -35 nodes 5000 nps 100000 time 50 pv e2e4 e7e5");
    }

    #[test]
    fn info_mate_without_pv_and_zero_time() {
        let info = UciOutgoing::Info(result(Score::from_mate_in(-4), &[], 0));
        assert_eq!(info.to_string(), "info depth 3 seldepth 7 score mate -2 nodes 5000 nps 5000000000 time 0");
    }

    #[test]
    fn score_formatting() {
        let cases = [
            (Score::from_centipawns(0), "cp 0"),
            (Score::from_centipawns(-35), "cp -35"),
            (Score::from_centipawns(i32::MAX), "cp 10000"),
            (Score::from_mate_in(1), "mate 1"),     // we mate on our next move
            (Score::from_mate_in(3), "mate 2"),     // our move, theirs, our mating move
            (Score::from_mate_in(4), "mate 2"),
            (Score::from_mate_in(-2), "mate -1"),   // their mating move comes after ours
            (Score::from_mate_in(-4), "mate -2"),
            (Score::from_mate_in(0), "mate 0"),     // already checkmated
        ];
        for (score, expected) in cases {
            assert_eq!(score.to_string(), expected, "{score:?}");
        }
    }
}
