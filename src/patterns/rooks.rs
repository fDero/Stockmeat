/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Square, Bitboard};
use crate::patterns::raywalk::*;
use konst::array::from_fn;

#[allow(long_running_const_eval)]
static ROOK_PATTERNS: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array(
        from_fn!(|landing_index| {
            let landing = Square::from_index(landing_index as u8);
            let file_distance = start.file().abs_diff(landing.file());
            let rank_distance = start.rank().abs_diff(landing.rank());
            (file_distance == 0) ^ (rank_distance == 0)
        })
    )
});

#[allow(long_running_const_eval)]
static ROOK_MASKS: [Bitboard; 64] = from_fn!(|index| {
    let start  = Square::from_index(index as u8);
    let (f, r) = (start.file(), start.rank());
    let file_ends = Bitboard::from_squares(&[Square::at(f, 0), Square::at(f, 7)]);
    let rank_ends = Bitboard::from_squares(&[Square::at(0, r), Square::at(7, r)]);
    ROOK_PATTERNS[index].without(file_ends).without(rank_ends)
});

static ROOK_RAYWALKS: RayWalks<4096> = precompute(|index| {
    let start = Square::from_index(index as u8);
    let rook_mask = ROOK_MASKS[index];
    std::array::from_fn(|key| {
        match key < (1 << rook_mask.bits().count_ones()) {
            true  => walk_straights(start, occupancy_decode(rook_mask, key as u16)),
            false => Bitboard::EMPTY,
        }
    })
});

pub fn get_rook_squares(start: Square, occupancy: Bitboard) -> Bitboard {
    let mask = ROOK_MASKS[start as usize];
    let rays = &ROOK_RAYWALKS[start as usize];
    rays[occupancy_encode(mask, occupancy) as usize]
}

/*---------------------------------------------------------------------+
|  =========================== UNIT-TESTS ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use Square::*;

    #[test]
    fn empty_board() {
        for start in [D4, A1, H8, E1] {
            assert_eq!(get_rook_squares(start, Bitboard::EMPTY).bits().count_ones(), 14);
        }
        let squares = get_rook_squares(D4, Bitboard::EMPTY);
        for square in [D1, D8, A4, H4] {
            assert!(squares.contains(square));
        }
    }

    #[test]
    fn stops_on_blockers() {
        let occupancy = Bitboard::from_squares(&[D6, B4, E5]);
        let expected = Bitboard::from_squares(&[D5, D6, D3, D2, D1, C4, B4, E4, F4, G4, H4]);
        assert_eq!(get_rook_squares(D4, occupancy), expected);
    }

    #[test]
    fn matches_raywalk() {
        let mut seed = 0x2545F4914F6CDD1Du64;
        for _ in 0..2000 {
            seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
            let occupancy = Bitboard::from_bits(seed & (seed >> 3));
            for index in 0..64 {
                let start = Square::from_index(index);
                assert_eq!(get_rook_squares(start, occupancy), walk_straights(start, occupancy));
            }
        }
    }
}
