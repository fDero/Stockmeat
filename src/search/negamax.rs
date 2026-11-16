/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::time::Duration;
use derive_more::Constructor;
use crate::search::{History, PrincipalVariation, TranspositionTable};
use crate::model::Move;

#[derive(Constructor)]
pub struct Negamax {
    history: History,
    settings: SearchSettings,
    principal: PrincipalVariation,
    transpositions: TranspositionTable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchSettings {
    pub searchmoves: Vec<Move>,
    pub ponder: bool,
    pub wtime: Option<Duration>,
    pub btime: Option<Duration>,
    pub winc: Option<Duration>,
    pub binc: Option<Duration>,
    pub movestogo: Option<u64>,
    pub depth: Option<u64>,
    pub nodes: Option<u64>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
}
