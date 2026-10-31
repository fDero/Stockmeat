/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use arrayvec::ArrayVec;

pub type SquareList<const N: usize> =
    ArrayVec<Square, N>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
    pub const fn file(self) -> u8 {
        self as u8 % 8
    }

    pub const fn rank(self) -> u8 {
        7 - self as u8 / 8
    }

    pub const fn rankarray(self) -> [Square; 8] {
        konst::array::from_fn!(|f| Square::at(f as u8, self.rank()))
    }

    pub const fn filearray(self) -> [Square; 8] {
        konst::array::from_fn!(|r| Square::at(self.file(), r as u8))
    }

    pub const fn equals(self, other: Square) -> bool {
        self as u8 == other as u8
    }

    pub const fn is_edge(self) -> bool {
        self.file() == 0 || self.file() == 7 || self.rank() == 0 || self.rank() == 7
    }

    pub const fn from_index(index: u8) -> Square {
        assert!(index < 64, "Invalid square index, must be between 0 and 63");
        unsafe { std::mem::transmute::<u8, Square>(index) }
    }

    pub const fn at(file: u8, rank: u8) -> Square {
        assert!(file < 8 && rank < 8, "Invalid coordinates, must be between 0 and 7");
        Square::from_index((7 - rank) * 8 + file)
    }

    pub const fn offset(self, df: i8, dr: i8) -> Option<Square> {
        let file = self.file() as i8 + df;
        let rank = self.rank() as i8 + dr;
        if 0 <= file && file < 8 && 0 <= rank && rank < 8 {
            Some(Square::at(file as u8, rank as u8))
        } else {
            None
        }
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
    use Square::*;

    #[test]
    fn file_and_rank() {
        for (square, file, rank) in [
            (A8, 0, 7),   (H8, 7, 7),
            (A1, 0, 0),   (E4, 4, 3),
        ] {
            assert_eq!((square.file(), square.rank()), (file, rank));
        }
    }

    #[test]
    fn at_roundtrip() {
        for index in 0..64 {
            let square = Square::from_index(index);
            assert_eq!(Square::at(square.file(), square.rank()), square);
        }
    }

    #[test]
    fn offset() {
        for (square, df, dr, expected) in [
            (E4,  1,  1, Some(F5)),   (A1, -1,  0, None),
            (H8,  0,  1, None),       (A1,  7,  7, Some(H8)),
        ] {
            assert_eq!(square.offset(df, dr), expected);
        }
    }

    #[test]
    fn rank_and_file_arrays() {
        assert_eq!(E4.rankarray(), [A4, B4, C4, D4, E4, F4, G4, H4]);
        assert_eq!(E4.filearray(), [E1, E2, E3, E4, E5, E6, E7, E8]);
    }
}
