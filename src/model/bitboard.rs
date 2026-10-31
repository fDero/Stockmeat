/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use getset::CopyGetters;
use konst::iter::for_each;
use crate::model::Square;
use Square::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Bitboard {
    bits: u64,
}

impl Bitboard {
    pub const EMPTY : Bitboard = Self::from_squares(&[]);

    pub const CORNERS : Bitboard = Self::from_squares(&[
        A1, A8, H1, H8
    ]);

    pub const EDGES : Bitboard = Self::from_squares(&[
        A1, A2, A3, A4, A5, A6, A7, A8,
            B1, C1, D1, E1, F1, G1, H1,
            H2, H3, H4, H5, H6, H7, H8,
            B8, C8, D8, E8, F8, G8,
    ]);

    pub const fn from_bits(bits: u64) -> Bitboard {
        Bitboard { bits }
    }

    pub const fn merge(self, other: Bitboard) -> Bitboard {
        Bitboard::from_bits(self.bits | other.bits)
    }

    pub const fn from_array(array : [bool; 64]) -> Bitboard {
        let mut bits : u64 = 0u64;
        for_each!(idx in 0..64 => bits |= (array[idx] as u64) << idx);
        Bitboard { bits }
    }

    pub const fn from_squares(list: &[Square]) -> Bitboard {
        let mut bits : u64 = 0u64;
        for_each!(&square in list => bits |= 1u64 << square as u8);
        Bitboard { bits }
    }

    pub const fn contains(self, square: Square) -> bool {
        (self.bits & (1u64 << square as u8)) != 0
    }

    pub const fn set(&mut self, square: Square) {
        self.bits |= 1u64 << square as u8
    }

    pub const fn unset(&mut self, square: Square) {
        self.bits &= !(1u64 << square as u8)
    }

    pub const fn without(self, other: Bitboard) -> Bitboard {
        Bitboard { bits : self.bits & !other.bits }
    }

    pub const fn diff(self, other: Bitboard) -> Bitboard {
        self.without(other)
    }

    pub const fn when(self, other: Bitboard) -> Bitboard {
        Bitboard { bits : self.bits & other.bits }
    }

    pub const fn with(self, list: &[Square]) -> Bitboard {
        Bitboard { bits: self.bits | Self::from_squares(list).bits }
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

    #[test]
    fn contains() {
        let board = Bitboard::from_squares(&[A1, E4]);
        for (square, expected) in [(A1, true), (E4, true), (E5, false), (H8, false)] {
            assert_eq!(board.contains(square), expected);
        }
    }

    #[test]
    fn set_and_unset() {
        let mut board = Bitboard::EMPTY;
        board.set(E4);
        assert!(board.contains(E4));
        board.unset(E4);
        assert_eq!(board, Bitboard::EMPTY);
    }

    #[test]
    fn with_and_without() {
        let board = Bitboard::EMPTY.with(&[A1, H8]);
        assert_eq!(board, Bitboard::from_squares(&[A1, H8]));
        assert_eq!(board.without(Bitboard::from_squares(&[A1])), Bitboard::from_squares(&[H8]));
    }

    #[test]
    fn diff_and_when() {
        let left  = Bitboard::from_squares(&[A1, E4, H8]);
        let right = Bitboard::from_squares(&[E4, D5]);
        assert_eq!(left.diff(right), Bitboard::from_squares(&[A1, H8]));
        assert_eq!(left.when(right), Bitboard::from_squares(&[E4]));
    }

    #[test]
    fn from_array() {
        let mut array = [false; 64];
        array[A1 as usize] = true;
        array[E4 as usize] = true;
        assert_eq!(Bitboard::from_array(array), Bitboard::from_squares(&[A1, E4]));
    }
}
