/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use derive_more::Neg;
use getset::CopyGetters;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, Neg)]
#[getset(get_copy = "pub")]
pub struct Score {
    raw: i32
}

impl Score {
    pub const MATE: i32 = 30_000;
    pub const MAX_CENTIPAWNS: i32 = 10_000;
    const MAX_MATE_DISTANCE: i32 = Self::MATE - Self::MAX_CENTIPAWNS - 1;

    pub fn from_mate_in_plies(plies: i32) -> Self {
        let plies = plies.clamp(-Self::MAX_MATE_DISTANCE, Self::MAX_MATE_DISTANCE);
        match plies {
            1.. => Self { raw: Self::MATE - plies },
            _ => Self { raw: -Self::MATE - plies },
        }
    }

    pub fn from_centipawns(centp: i32) -> Self {
        Self { raw: centp.clamp(-Self::MAX_CENTIPAWNS, Self::MAX_CENTIPAWNS) }
    }

    pub fn mate_in_plies(self) -> Option<i32> {
        match self.raw {
            raw if raw > Self::MAX_CENTIPAWNS => Some(Self::MATE - raw),
            raw if raw < -Self::MAX_CENTIPAWNS => Some(-Self::MATE - raw),
            _ => None,
        }
    }

    pub fn centipawns(self) -> Option<i32> {
        match self.mate_in_plies() {
            Some(_) => None,
            None => Some(self.raw),
        }
    }

    pub fn bubble_up(self) -> Self {
        match self.raw {
            raw if raw >  Self::MAX_CENTIPAWNS => Self { raw: raw - 1 },
            raw if raw < -Self::MAX_CENTIPAWNS => Self { raw: raw + 1 },
            _ => self,
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
    fn centipawns_round_trip() {
        for cp in [0, 1, -1, 35, -35, 9_999, -9_999, Score::MAX_CENTIPAWNS, -Score::MAX_CENTIPAWNS] {
            let score = Score::from_centipawns(cp);
            assert_eq!(score.centipawns(), Some(cp), "{cp}");
            assert_eq!(score.mate_in_plies(), None, "{cp}");
        }
    }

    #[test]
    fn centipawns_are_clamped() {
        assert_eq!(Score::from_centipawns(i32::MAX).centipawns(), Some(Score::MAX_CENTIPAWNS));
        assert_eq!(Score::from_centipawns(i32::MIN).centipawns(), Some(-Score::MAX_CENTIPAWNS));
        assert_eq!(Score::from_centipawns(25_000).mate_in_plies(), None);
        assert_eq!(Score::from_centipawns(-25_000).mate_in_plies(), None);
    }

    #[test]
    fn mate_round_trip() {
        for plies in [1, 2, 3, 50, -1, -2, -3, -50, 0] {
            let score = Score::from_mate_in_plies(plies);
            assert_eq!(score.mate_in_plies(), Some(plies), "{plies}");
            assert_eq!(score.centipawns(), None, "{plies}");
        }
    }

    #[test]
    fn mate_distance_is_clamped() {
        let far = Score::from_mate_in_plies(i32::MAX);
        let far_mated = Score::from_mate_in_plies(i32::MIN);
        assert!(far.mate_in_plies().unwrap() > 0);
        assert!(far_mated.mate_in_plies().unwrap() < 0);
        assert!(far > Score::from_centipawns(i32::MAX));
        assert!(far_mated < Score::from_centipawns(i32::MIN));
    }

    #[test]
    fn ordering_prefers_faster_mates_and_slower_losses() {
        let ordered = [
            Score::from_mate_in_plies(0),
            Score::from_mate_in_plies(-1),
            Score::from_mate_in_plies(-5),
            Score::from_centipawns(i32::MIN),
            Score::from_centipawns(-35),
            Score::from_centipawns(0),
            Score::from_centipawns(35),
            Score::from_centipawns(i32::MAX),
            Score::from_mate_in_plies(5),
            Score::from_mate_in_plies(1),
        ];
        assert!(ordered.windows(2).all(|pair| pair[0] < pair[1]), "{ordered:?}");
    }

    #[test]
    fn negation_flips_the_perspective() {
        for cp in [0, 35, -35, Score::MAX_CENTIPAWNS, -Score::MAX_CENTIPAWNS] {
            assert_eq!(-Score::from_centipawns(cp), Score::from_centipawns(-cp), "{cp}");
        }
        for plies in [1, 3, 50, -1, -3, -50] {
            assert_eq!(-Score::from_mate_in_plies(plies), Score::from_mate_in_plies(-plies), "{plies}");
        }
        let score = Score::from_mate_in_plies(4);
        assert_eq!(-(-score), score);
    }

    #[test]
    fn bubble_up_leaves_centipawns_untouched() {
        for cp in [0, 35, -35, Score::MAX_CENTIPAWNS, -Score::MAX_CENTIPAWNS] {
            let score = Score::from_centipawns(cp);
            assert_eq!(score.bubble_up(), score, "{cp}");
        }
    }

    #[test]
    fn bubble_up_pushes_mates_one_ply_further() {
        assert_eq!(Score::from_mate_in_plies(3).bubble_up(), Score::from_mate_in_plies(4));
        assert_eq!(Score::from_mate_in_plies(-3).bubble_up(), Score::from_mate_in_plies(-4));
        assert_eq!(Score::from_mate_in_plies(0).bubble_up(), Score::from_mate_in_plies(-1));
    }
}
