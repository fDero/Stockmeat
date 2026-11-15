/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::scores::Score;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ApproxScore {
    Exact(Score),
    LessThan(Score),
    GreaterThan(Score),
}

impl ApproxScore {
    pub fn score(self) -> Score {
        match self {
            ApproxScore::Exact(score) => score,
            ApproxScore::LessThan(score) => score,
            ApproxScore::GreaterThan(score) => score,
        }
    }

    pub fn max(self, other: ApproxScore) -> ApproxScore {
        let bounds = |approx| match approx {
            ApproxScore::Exact(score)       => (Some(score), Some(score)),
            ApproxScore::LessThan(score)    => (None, Some(score)),
            ApproxScore::GreaterThan(score) => (Some(score), None),
        };
        let ((low_a, high_a), (low_b, high_b)) = (bounds(self), bounds(other));
        let lower = low_a.max(low_b);
        let upper = high_a.zip(high_b).map(|(a, b)| a.max(b));
        match (lower, upper) {
            (Some(low), Some(high)) if low == high => ApproxScore::Exact(low),
            (Some(low), None)                      => ApproxScore::GreaterThan(low),
            (_, Some(high))                        => ApproxScore::LessThan(high),
            (None, None)                           => unreachable!("one of the bounds is always finite"),
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
    use ApproxScore::*;

    fn cp(centipawns: i32) -> Score {
        Score::from_centipawns(centipawns)
    }

    #[test]
    fn fail_high_sibling_dominates() {
        assert_eq!(Exact(cp(20)).max(GreaterThan(cp(50))), GreaterThan(cp(50)));
        assert_eq!(Exact(cp(80)).max(GreaterThan(cp(50))), GreaterThan(cp(80)));
        assert_eq!(LessThan(cp(90)).max(GreaterThan(cp(50))), GreaterThan(cp(50)));
    }

    #[test]
    fn upper_bounds_combine_into_the_larger_one() {
        assert_eq!(LessThan(cp(5)).max(LessThan(cp(-30))), LessThan(cp(5)));
    }

    #[test]
    fn unrepresentable_range_is_widened_to_its_upper_bound() {
        assert_eq!(Exact(cp(5)).max(LessThan(cp(10))), LessThan(cp(10)));
    }
}
