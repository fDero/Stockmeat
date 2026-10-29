/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

mod format;
mod model;
mod moves;
mod patterns;

use format::FenDecode;
use model::Game;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() -> anyhow::Result<()> {
    let game = Game::fen_decode(STARTING_FEN)?;
    print!("{game}");
    Ok(())
}
