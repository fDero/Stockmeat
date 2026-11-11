/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use derive_more::Constructor;
use getset::CopyGetters;
use crate::search::PositionHash;
use crate::model::Move;
use super::ApproxScore;

pub struct TranspositionTable {
     entries: Box<[Option<TableEntry>]>,
}

#[derive(Copy, Clone, Debug, CopyGetters, Constructor)]
#[getset(get_copy = "pub")]
pub struct TableEntry {
    full_hash:    PositionHash,
    best_move:    Option<Move>,
    approx_score: ApproxScore,
    search_depth: u8,
    generation:   u8,
}

impl TranspositionTable {
    pub fn with_megabytes(megabytes: usize) -> Self {
        let len = ((megabytes << 20) / size_of::<Option<TableEntry>>()).max(1);
        Self { entries: vec![None; len].into_boxed_slice() }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn probe(&self, hash: PositionHash) -> Option<TableEntry> {
        self.entries[self.index(hash)].filter(|entry| entry.full_hash == hash)
    }

    pub fn store(&mut self, mut entry: TableEntry) {
        let slot = &mut self.entries[self.index(entry.full_hash)];
        if let Some(old) = *slot {
            if old.generation == entry.generation && entry.search_depth < old.search_depth {
                return;
            }
            if old.full_hash == entry.full_hash {
                entry.best_move = entry.best_move.or(old.best_move);
            }
        }
        *slot = Some(entry);
    }

    pub fn clear(&mut self) {
        self.entries.fill(None);
    }

    fn index(&self, hash: PositionHash) -> usize {
        ((hash.raw() as u128 * self.entries.len() as u128) >> 64) as usize
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
    use crate::format::FenDecode;
    use crate::model::{Game, Square};
    use crate::scores::Score;
    use Square::*;

    fn hash(fen: &str) -> PositionHash {
        PositionHash::for_game(&Game::fen_decode(fen).unwrap())
    }

    fn exact(cp: i32) -> ApproxScore {
        ApproxScore::Exact(Score::from_centipawns(cp))
    }

    const FEN_A: &str = "4k3/8/8/8/8/8/8/R3K3 w - - 0 1";
    const FEN_B: &str = "4k3/8/8/8/8/8/8/R3K3 b - - 0 1";

    #[test]
    fn store_then_probe() {
        let mut table = TranspositionTable::with_megabytes(1);
        let mov = Move::from_squares(A1, A8);
        table.store(TableEntry::new(hash(FEN_A), Some(mov), exact(42), 5, 0));
        let entry = table.probe(hash(FEN_A)).unwrap();
        assert_eq!(entry.best_move(), Some(mov));
        assert_eq!(entry.approx_score(), exact(42));
        assert_eq!(entry.search_depth(), 5);
        assert!(table.probe(hash(FEN_B)).is_none());
    }

    #[test]
    fn index_mismatch_is_rejected() {
        let mut table = TranspositionTable::with_megabytes(0);
        assert_eq!(table.len(), 1);
        table.store(TableEntry::new(hash(FEN_A), None, exact(1), 3, 0));
        assert!(table.probe(hash(FEN_B)).is_none());
    }

    #[test]
    fn shallower_entries_do_not_replace_deeper_ones_in_same_search() {
        let mut table = TranspositionTable::with_megabytes(0);
        table.store(TableEntry::new(hash(FEN_A), None, exact(1), 6, 0));
        table.store(TableEntry::new(hash(FEN_B), None, exact(2), 2, 0));
        assert!(table.probe(hash(FEN_A)).is_some());
        assert!(table.probe(hash(FEN_B)).is_none());
    }

    #[test]
    fn stale_entries_are_always_replaced() {
        let mut table = TranspositionTable::with_megabytes(0);
        table.store(TableEntry::new(hash(FEN_A), None, exact(1), 6, 0));
        table.store(TableEntry::new(hash(FEN_B), None, exact(2), 1, 1));
        assert!(table.probe(hash(FEN_A)).is_none());
        assert_eq!(table.probe(hash(FEN_B)).unwrap().generation(), 1);
    }

    #[test]
    fn best_move_survives_moveless_update() {
        let mut table = TranspositionTable::with_megabytes(1);
        let mov = Move::from_squares(A1, A8);
        table.store(TableEntry::new(hash(FEN_A), Some(mov), exact(10), 3, 0));
        table.store(TableEntry::new(hash(FEN_A), None, ApproxScore::LessThan(Score::from_centipawns(0)), 4, 0));
        let entry = table.probe(hash(FEN_A)).unwrap();
        assert_eq!(entry.best_move(), Some(mov));
        assert_eq!(entry.search_depth(), 4);
    }

    #[test]
    fn clear_empties_the_table() {
        let mut table = TranspositionTable::with_megabytes(1);
        table.store(TableEntry::new(hash(FEN_A), None, exact(1), 1, 0));
        table.clear();
        assert!(table.probe(hash(FEN_A)).is_none());
    }
}
