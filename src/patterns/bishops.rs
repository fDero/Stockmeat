/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Square, Bitboard};
use crate::patterns::raywalk::*;
use konst::array::from_fn;

#[allow(long_running_const_eval)]
pub static BISHOP_PATTERNS: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array(
        from_fn!(|landing_index| {
            let landing = Square::from_index(landing_index as u8);
            let file_distance = start.file().abs_diff(landing.file());
            let rank_distance = start.rank().abs_diff(landing.rank());
            file_distance == rank_distance && !landing.equals(start)
        })
    )
});

#[allow(long_running_const_eval)]
static BISHOP_MASKS: [Bitboard; 64] = from_fn!(|index| {
    BISHOP_PATTERNS[index].without(Bitboard::EDGES)
});

static BISHOP_RAYWALKS: RayWalks<512> = precompute(|index| {
    let start = Square::from_index(index as u8);
    let bishop_mask = BISHOP_MASKS[index];
    std::array::from_fn(|key| {
        let occupancy = occupancy_decode(bishop_mask, key as u16);
        match key < (1 << bishop_mask.bits().count_ones()) {
            true  => walk_diagonals(start, occupancy),
            false => Bitboard::EMPTY,
        }
    })
});

pub fn get_bishop_squares(start: Square, occupancy: Bitboard) -> Bitboard {
    let mask = BISHOP_MASKS[start as usize];
    let rays = &BISHOP_RAYWALKS[start as usize];
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
        assert_eq!(get_bishop_squares(D4, Bitboard::EMPTY).bits().count_ones(), 13);
        assert_eq!(get_bishop_squares(A1, Bitboard::EMPTY).bits().count_ones(), 7);
        let squares = get_bishop_squares(D4, Bitboard::EMPTY);
        for square in [A1, H8, A7, G1] {
            assert!(squares.contains(square));
        }
    }

    #[test]
    fn stops_on_blockers() {
        let occupancy = Bitboard::from_squares(&[F6, B2, D5]);
        let expected = Bitboard::from_squares(&[E5, F6, C3, B2, C5, B6, A7, E3, F2, G1]);
        assert_eq!(get_bishop_squares(D4, occupancy), expected);
    }

    #[test]
    fn matches_raywalk() {
        let mut seed = 0x9E3779B97F4A7C15u64;
        for _ in 0..2000 {
            seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
            let occupancy = Bitboard::from_bits(seed & (seed >> 3));
            for index in 0..64 {
                let start = Square::from_index(index);
                assert_eq!(get_bishop_squares(start, occupancy), walk_diagonals(start, occupancy));
            }
        }
    }
}
