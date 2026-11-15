/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::fmt::{Display, Formatter, Result};
use crate::scores::Score;
use crate::search::{ApproxScore, PrincipalVariation, SearchResult};
use crate::search::ApproxScore::{Exact, GreaterThan, LessThan};
use crate::model::*;

#[derive(Clone, Debug)]
pub enum UciOutgoing<'a> {
    Identify, UciOk, ReadyOk,
    BestMove(MoveList<2>),
    Info(SearchResult, &'a PrincipalVariation),
}

impl Display for Score {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match (self.mate_in_plies(), self.centipawns()) {
            (Some(plies), _) => write!(formatter, "mate {}", (plies + 1).div_euclid(2)),
            (None, centipawns) => write!(formatter, "cp {}", centipawns.unwrap_or_default()),
        }
    }
}

impl<const N: usize> Display for PrincipalVariation<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        for (index, mov) in self.iter().enumerate() {
            match index {
                0 => write!(formatter, "{mov}")?,
                _ => write!(formatter, " {mov}")?,
            }
        }
        Ok(())
    }
}

impl Display for ApproxScore {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Exact(score) => write!(formatter, "{}", score),
            LessThan(score) => write!(formatter, "{} upperbound", score),
            GreaterThan(score) => write!(formatter, "{} lowerbound", score),
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let millis = self.search_duration().as_millis();
        let micros = self.search_duration().as_micros().max(1);
        let nps = self.nodes_counter() as u128 * 1_000_000 / micros;
        write!(formatter, "info depth {} seldepth {} score {} nodes {} nps {} time {}",
            self.minimum_depth(), self.maximum_depth(), self.updated_score(),
            self.nodes_counter(), nps, millis
        )
    }
}

impl Display for UciOutgoing<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let identity = "id name Stockmeat\nid author fDero";
        match self {
            UciOutgoing::UciOk => write!(formatter, "uciok"),
            UciOutgoing::ReadyOk => write!(formatter, "readyok"),
            UciOutgoing::Identify => write!(formatter, "{}", identity),
            UciOutgoing::Info(result, pv) => match pv.principal().is_empty() {
                true  => write!(formatter, "{result}"),
                false => write!(formatter, "{result} pv {pv}"),
            },
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
    use std::time::Duration;
    use Square::*;
    use super::*;

    fn result(score: Score, millis: u64) -> SearchResult {
        SearchResult::new(Duration::from_millis(millis), Exact(score), 3, 7, 5000)
    }

    fn variation(moves: &[Move]) -> Box<PrincipalVariation> {
        let mut pv = Box::new(PrincipalVariation::new());
        for (ply, &mov) in moves.iter().enumerate().rev() {
            pv.update(ply, mov);
        }
        pv
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
        let pv = variation(&[Move::from_squares(E2, E4), Move::from_squares(E7, E5)]);
        let info = UciOutgoing::Info(result(Score::from_centipawns(-35), 50), &pv);
        assert_eq!(info.to_string(), "info depth 3 seldepth 7 score cp -35 nodes 5000 nps 100000 time 50 pv e2e4 e7e5");
    }

    #[test]
    fn info_mate_without_pv_and_zero_time() {
        let pv = variation(&[]);
        let info = UciOutgoing::Info(result(Score::from_mate_in_plies(-4), 0), &pv);
        assert_eq!(info.to_string(), "info depth 3 seldepth 7 score mate -2 nodes 5000 nps 5000000000 time 0");
    }

    #[test]
    fn bounds_are_reported() {
        assert_eq!(Exact(Score::from_centipawns(35)).to_string(), "cp 35");
        assert_eq!(LessThan(Score::from_centipawns(35)).to_string(), "cp 35 upperbound");
        assert_eq!(GreaterThan(Score::from_mate_in_plies(3)).to_string(), "mate 2 lowerbound");
    }

    #[test]
    fn score_formatting() {
        let cases = [
            (Score::from_centipawns(0), "cp 0"),
            (Score::from_centipawns(-35), "cp -35"),
            (Score::from_centipawns(i32::MAX), "cp 10000"),
            (Score::from_mate_in_plies(1), "mate 1"),     // we mate on our next move
            (Score::from_mate_in_plies(3), "mate 2"),     // our move, theirs, our mating move
            (Score::from_mate_in_plies(4), "mate 2"),
            (Score::from_mate_in_plies(-2), "mate -1"),   // their mating move comes after ours
            (Score::from_mate_in_plies(-4), "mate -2"),
            (Score::from_mate_in_plies(0), "mate 0"),     // already checkmated
        ];
        for (score, expected) in cases {
            assert_eq!(score.to_string(), expected, "{score:?}");
        }
    }
}
