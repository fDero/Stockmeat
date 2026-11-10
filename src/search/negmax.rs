use super::Score;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ApproxScore {
    Exact(Score),
    LessThan(Score),
    GreaterThan(Score),
}
