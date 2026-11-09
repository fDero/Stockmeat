/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

mod format;
mod model;
mod moves;
mod patterns;
mod search;

use format::{FenDecode, STARTING_FEN};
use model::Game;

fn main() -> anyhow::Result<()> {
    let game = Game::fen_decode(STARTING_FEN)?;
    print!("{game}");
    Ok(())
}
