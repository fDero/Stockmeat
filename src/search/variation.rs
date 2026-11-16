/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::fmt::{Debug, Formatter, Result};
use crate::model::Move;

#[derive(Clone)]
pub struct PrincipalVariation<const N: usize = 300> {
    moves:   [[Move; N]; N],
    lengths: [usize; N],
}

impl<const N: usize> PrincipalVariation<N> {
    pub fn new() -> Self {
        Self { moves: [[Move::NULL; N]; N], lengths: [0; N] }
    }

    pub fn clear(&mut self, ply: usize) {
        self.lengths[ply] = 0;
    }

    pub fn update(&mut self, ply: usize, mov: Move) {
        let (upper, lower) = self.moves.split_at_mut(ply + 1);
        let child_len = self.lengths[ply + 1];
        upper[ply][0] = mov;
        upper[ply][1..=child_len].copy_from_slice(&lower[0][..child_len]);
        self.lengths[ply] = child_len + 1;
    }

    pub fn principal(&self) -> &[Move] {
        &self.moves[0][..self.lengths[0]]
    }

    pub fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        self.principal().iter().copied()
    }
}

impl<const N: usize> Debug for PrincipalVariation<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}
