/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Square, Bitboard};
use crate::patterns::{get_bishop_squares, get_rook_squares};
use konst::cmp::{min, max};
use konst::array::from_fn;

#[allow(long_running_const_eval)]
pub static KNIGHT_PATTERNS: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array(
        from_fn!(|landing_index| {
            let landing = Square::from_index(landing_index as u8);
            let horiz_delta = start.rank().abs_diff(landing.rank());
            let vert_delta  = start.file().abs_diff(landing.file());
            let maxd_valid  = max!(vert_delta, horiz_delta) == 2;
            let mind_valid  = min!(vert_delta, horiz_delta) == 1;
            maxd_valid && mind_valid
        })
    )
});

#[allow(long_running_const_eval)]
pub static KING_PATTERNS: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array(
        from_fn!(|landing_index| {
            let landing = Square::from_index(landing_index as u8);
            let horiz_delta = start.rank().abs_diff(landing.rank());
            let vert_delta  = start.file().abs_diff(landing.file());
            let maxd_valid  = max!(vert_delta, horiz_delta) == 1;
            let mind_valid  = min!(vert_delta, horiz_delta) <= 1;
            maxd_valid && mind_valid
        })
    )
});

pub fn get_queen_squares(start: Square, occupancy: Bitboard) -> Bitboard {
    get_bishop_squares(start, occupancy).merge(
        get_rook_squares(start, occupancy)
    )
}

pub fn get_king_squares(start: Square) -> Bitboard {
    KING_PATTERNS[start as usize]
}

pub fn get_knight_squares(start: Square) -> Bitboard {
    KNIGHT_PATTERNS[start as usize]
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
    fn knight() {
        assert_eq!(get_knight_squares(A1), Bitboard::from_squares(&[B3, C2]));
        assert_eq!(get_knight_squares(H8), Bitboard::from_squares(&[G6, F7]));
        assert_eq!(get_knight_squares(D4), Bitboard::from_squares(&[C2, E2, B3, F3, B5, F5, C6, E6]));
    }

    #[test]
    fn king() {
        assert_eq!(get_king_squares(A1), Bitboard::from_squares(&[A2, B1, B2]));
        assert_eq!(get_king_squares(E1), Bitboard::from_squares(&[D1, F1, D2, E2, F2]));
        assert_eq!(get_king_squares(D4).bits().count_ones(), 8);
    }

    #[test]
    fn queen() {
        assert_eq!(get_queen_squares(D4, Bitboard::EMPTY).bits().count_ones(), 27);
        let occupancy = Bitboard::from_squares(&[D6, F6]);
        let expected = get_rook_squares(D4, occupancy).merge(get_bishop_squares(D4, occupancy));
        assert_eq!(get_queen_squares(D4, occupancy), expected);
        assert!(!get_queen_squares(D4, occupancy).contains(D7));
        assert!(!get_queen_squares(D4, occupancy).contains(G7));
    }
}
