/*---------------------------------------------------------------------+
|  ========================== RUST-MODULE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

mod score;
mod negmax;
mod ttable;
mod hashing;

pub use hashing::PositionHash;
pub use score::Score;
pub use negmax::ApproxScore;
