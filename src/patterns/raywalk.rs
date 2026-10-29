/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Square, Bitboard};
use std::sync::{LazyLock};

pub type RayWalks<const N: usize> =
    LazyLock<Box<[[Bitboard; N]; 64]>>;

pub fn occupancy_decode(mask: Bitboard, key: u16) -> Bitboard {
    let bits = key as u64;
    Bitboard::from_bits(unsafe { core::arch::x86_64::_pdep_u64(bits, mask.bits()) })
}

pub fn occupancy_encode(mask: Bitboard, occupancy: Bitboard) -> u16 {
    let bits = occupancy.bits();
    (unsafe { core::arch::x86_64::_pext_u64(bits, mask.bits()) }) as u16
}

pub const fn precompute<const N: usize, F: Fn(usize) -> [Bitboard; N]>(f: F) -> RayWalks<N> {
    const { assert!(size_of::<F>() == 0, "precompute needs a closure that captures nothing") };
    std::mem::forget(f);
    LazyLock::new(|| {
        let f = unsafe { std::mem::zeroed::<F>() };
        (0..64).map(f).collect::<Box<[_]>>().try_into().ok().unwrap()
    })
}

fn walk_offset(start : Square, occ : Bitboard, xy : (i8, i8)) -> Bitboard {
    Bitboard::from_bits(
        konst::iter::eval!(
            1i8..8, map_while(|step| start.offset(step * xy.0, step * xy.1)),
            fold(0u64, |bits, landing| match bits & occ.bits() == 0 {
                true  => bits | 1u64 << landing as u8,
                false => bits,
            })
        )
    )
}

pub fn walk_diagonals(start : Square, occupancy : Bitboard) -> Bitboard {
    Bitboard::from_bits(
        konst::iter::eval!(
            [(1,1), (-1,-1), (1,-1), (-1,1)],
            map(|offset| walk_offset(start, occupancy, offset)),
            fold(0u64, |bits, raywalk| { bits | raywalk.bits() })
        )
    )
}

pub fn walk_straights(start : Square, occupancy : Bitboard) -> Bitboard {
    Bitboard::from_bits(
        konst::iter::eval!(
            [(0,1), (0,-1), (1,0), (-1,0)],
            map(|offset| walk_offset(start, occupancy, offset)),
            fold(0u64, |bits, raywalk| { bits | raywalk.bits() })
        )
    )
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

    static INDEX_TABLE: RayWalks<2> = precompute(|index| {
        [Bitboard::from_bits(index as u64), Bitboard::EMPTY]
    });

    #[test]
    fn occupancy_roundtrip() {
        let mask = Bitboard::from_squares(&[B2, D4, E5, G7]);
        for key in 0..16u16 {
            let occupancy = occupancy_decode(mask, key);
            assert_eq!(occupancy.without(mask), Bitboard::EMPTY);
            assert_eq!(occupancy_encode(mask, occupancy), key);
        }
        let occupancy = Bitboard::from_squares(&[A1, D4, G7, H8]);
        let key = occupancy_encode(mask, occupancy);
        assert_eq!(occupancy_decode(mask, key), occupancy.when(mask));
    }

    #[test]
    fn straights() {
        assert_eq!(walk_straights(D4, Bitboard::EMPTY).bits().count_ones(), 14);
        assert_eq!(walk_straights(A1, Bitboard::EMPTY).bits().count_ones(), 14);
        let occupancy = Bitboard::from_squares(&[D6, B4]);
        let expected = Bitboard::from_squares(&[D5, D6, D3, D2, D1, C4, B4, E4, F4, G4, H4]);
        assert_eq!(walk_straights(D4, occupancy), expected);
    }

    #[test]
    fn diagonals() {
        assert_eq!(walk_diagonals(D4, Bitboard::EMPTY).bits().count_ones(), 13);
        assert_eq!(walk_diagonals(A1, Bitboard::EMPTY).bits().count_ones(), 7);
        let occupancy = Bitboard::from_squares(&[F6, B2]);
        let expected = Bitboard::from_squares(&[E5, F6, C3, B2, C5, B6, A7, E3, F2, G1]);
        assert_eq!(walk_diagonals(D4, occupancy), expected);
    }

    #[test]
    fn start_square_occupancy_is_ignored() {
        let occupancy = Bitboard::from_squares(&[D4]);
        assert_eq!(walk_straights(D4, occupancy), walk_straights(D4, Bitboard::EMPTY));
        assert_eq!(walk_diagonals(D4, occupancy), walk_diagonals(D4, Bitboard::EMPTY));
    }

    #[test]
    fn adjacent_blockers() {
        let occupancy = Bitboard::from_squares(&[D5, D3, C4, E4, C5, E5, C3, E3]);
        assert_eq!(walk_straights(D4, occupancy), Bitboard::from_squares(&[D5, D3, C4, E4]));
        assert_eq!(walk_diagonals(D4, occupancy), Bitboard::from_squares(&[C5, E5, C3, E3]));
    }

    #[test]
    fn precompute_fills_every_square() {
        for index in 0..64 {
            assert_eq!(INDEX_TABLE[index], [Bitboard::from_bits(index as u64), Bitboard::EMPTY]);
        }
    }
}
