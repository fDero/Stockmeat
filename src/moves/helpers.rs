/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;
use crate::patterns::*;
use crate::model::MoveList;

use Piece::*;

impl Game {
    pub fn get_pawn_moves(self, _start: Square) -> MoveList<12> {
        todo!()
    }

    pub fn get_king_moves(self, _start: Square) -> MoveList<8> {
        todo!()
    }
}
