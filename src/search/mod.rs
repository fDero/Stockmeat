/*---------------------------------------------------------------------+
|  ========================== RUST-MODULE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

mod approx;
mod result;
mod ttable;
mod hashing;
mod status;
mod history;

pub use hashing::PositionHash;
pub use approx::ApproxScore;
pub use result::SearchResult;
pub use status::GameStatus;
pub use history::History;
