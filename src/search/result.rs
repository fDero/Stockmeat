/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::time::{Duration, Instant};
use derive_more::Constructor;
use getset::CopyGetters;
use crate::model::{Color, Game};
use super::{GameStatus, History};
use crate::scores::Score;
use super::ApproxScore;

#[derive(Clone, Debug, PartialEq, Eq, CopyGetters, Constructor)]
#[getset(get_copy = "pub")]
pub struct SearchResult {
    search_duration: Duration,
    updated_score:   ApproxScore,
    minimum_depth:   u32,
    maximum_depth:   u32,
    nodes_counter:   u64,
}

impl SearchResult {
    pub fn from_leaf(game: Game, history: &History) -> SearchResult {
        let chrono = Instant::now();
        let mut heuristic = game.board.dynamic_combined_heuristic();
        if game.current_turn == Color::Black {
            heuristic = -heuristic;
        }
        let score = match game.status(history) {
            GameStatus::Checkmate(_) => Score::from_mate_in(0),
            GameStatus::Ongoing(_)   => Score::from_centipawns(heuristic),
            _                        => Score::from_centipawns(0),
        };
        let exact = ApproxScore::Exact(score);
        SearchResult::new(chrono.elapsed(), exact, 0, 0, 1)
    }

    pub fn merge(self, sibling: SearchResult) -> SearchResult {
        let current_raw_score = self.updated_score.score();
        let sibling_raw_score = sibling.updated_score.score();
        let updated_score = self.updated_score.max(sibling.updated_score);
        let (best, other) = match sibling_raw_score > current_raw_score {
            true  => (sibling, self),
            false => (self, sibling),
        };
        SearchResult {
            updated_score,
            search_duration: best.search_duration + other.search_duration,
            minimum_depth:   best.minimum_depth.min(other.minimum_depth),
            maximum_depth:   best.maximum_depth.max(other.maximum_depth),
            nodes_counter:   best.nodes_counter + other.nodes_counter,
            ..best
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
    use crate::format::FenDecode;
    use ApproxScore::*;

    fn cp(centipawns: i32) -> Score {
        Score::from_centipawns(centipawns)
    }

    fn result(score: ApproxScore, millis: u64, depths: (u32, u32), nodes: u64) -> SearchResult {
        SearchResult::new(Duration::from_millis(millis), score, depths.0, depths.1, nodes)
    }

    fn scored(score: ApproxScore) -> SearchResult {
        result(score, 0, (1, 1), 1)
    }

    fn leaf(fen: &str) -> SearchResult {
        SearchResult::from_leaf(Game::fen_decode(fen).unwrap(), &History::default())
    }

    #[test]
    fn leaf_scores_terminal_positions() {
        let fools_mate = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
        assert_eq!(leaf(fools_mate).updated_score(), Exact(Score::from_mate_in(0)));
        assert_eq!(leaf("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1").updated_score(), Exact(cp(0)));
    }

    #[test]
    fn better_score_wins() {
        let merged = scored(Exact(cp(10))).merge(scored(Exact(cp(30))));
        assert_eq!(merged.updated_score(), Exact(cp(30)));
    }

    #[test]
    fn merge_is_symmetric_on_the_winner() {
        let a = result(Exact(cp(10)), 5, (3, 5), 100);
        let b = result(Exact(cp(30)), 7, (4, 9), 200);
        assert_eq!(a.clone().merge(b.clone()), b.merge(a));
    }

    #[test]
    fn work_is_accumulated() {
        let merged = result(Exact(cp(10)), 5, (3, 5), 100).merge(result(Exact(cp(30)), 7, (4, 9), 200));
        assert_eq!(merged.search_duration(), Duration::from_millis(12));
        assert_eq!(merged.minimum_depth(), 3);
        assert_eq!(merged.maximum_depth(), 9);
        assert_eq!(merged.nodes_counter(), 300);
    }

    #[test]
    fn mates_beat_centipawns() {
        let mate = Exact(Score::from_mate_in(3));
        let merged = scored(Exact(cp(Score::MAX_CENTIPAWNS))).merge(scored(mate));
        assert_eq!(merged.updated_score(), mate);
    }

    #[test]
    fn fail_low_sibling_does_not_spoil_an_exact_score() {
        let merged = scored(LessThan(cp(5))).merge(scored(Exact(cp(20))));
        assert_eq!(merged.updated_score(), Exact(cp(20)));
    }
}
