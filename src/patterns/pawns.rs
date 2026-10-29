/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::{Bitboard, Color, Square};
use konst::array::{from_fn};

#[allow(long_running_const_eval)]
static PAWN_PUSHES: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array(
        from_fn!(|landing_index| {
            let landing = Square::from_index(landing_index as u8);
            let file_distance = start.file().abs_diff(landing.file());
            let rank_distance = start.rank().abs_diff(landing.rank());
            let single_push = file_distance == 0 && rank_distance == 1;
            let double_push = file_distance == 0 && rank_distance == 2;
            single_push || double_push && (start.rank() == 1 || start.rank() == 6)
        })
    )
});

#[allow(long_running_const_eval)]
static PAWN_CAPTURES: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array(
        from_fn!(|landing_index| {
            let landing = Square::from_index(landing_index as u8);
            let file_distance = start.file().abs_diff(landing.file());
            let rank_distance = start.rank().abs_diff(landing.rank());
            file_distance == 1 && rank_distance == 1
        })
    )
});

#[allow(long_running_const_eval)]
static WHITE_PAWNS_MASK: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array( from_fn!(|landing_index| {
        start.rank() > Square::from_index(landing_index as u8).rank()
    }))
});

#[allow(long_running_const_eval)]
static BLACK_PAWNS_MASK: [Bitboard; 64] = from_fn!(|index| {
    let start = Square::from_index(index as u8);
    Bitboard::from_array( from_fn!(|landing_index| {
        start.rank() < Square::from_index(landing_index as u8).rank()
    }))
});

pub fn get_wpawn_squares(start: Square, occupancy: Bitboard) -> Bitboard {
    let blockers = PAWN_PUSHES[start as usize].when(occupancy);
    let shadows = Bitboard::from_bits(blockers.bits() >> 8);
    let pushes = PAWN_PUSHES[start as usize].diff(occupancy).diff(shadows);
    let captures = PAWN_CAPTURES[start as usize].when(occupancy);
    let color_mask = WHITE_PAWNS_MASK[start as usize];
    pushes.diff(color_mask).merge(captures.diff(color_mask))
}

pub fn get_bpawn_squares(start: Square, occupancy: Bitboard) -> Bitboard {
    let blockers = PAWN_PUSHES[start as usize].when(occupancy);
    let shadows = Bitboard::from_bits(blockers.bits() << 8);
    let pushes = PAWN_PUSHES[start as usize].diff(occupancy).diff(shadows);
    let captures = PAWN_CAPTURES[start as usize].when(occupancy);
    let color_mask = BLACK_PAWNS_MASK[start as usize];
    pushes.diff(color_mask).merge(captures.diff(color_mask))
}

pub fn get_pawn_squares(start: Square, color: Color, occupancy: Bitboard) -> Bitboard {
    match color {
        Color::White => get_wpawn_squares(start, occupancy),
        Color::Black => get_bpawn_squares(start, occupancy),
    }
}

pub fn get_pawn_attacks(start: Square, color: Color) -> Bitboard {
    let color_mask = match color {
        Color::White => WHITE_PAWNS_MASK[start as usize],
        Color::Black => BLACK_PAWNS_MASK[start as usize],
    };
    PAWN_CAPTURES[start as usize].diff(color_mask)
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
    fn white_pushes() {
        assert_eq!(get_wpawn_squares(E2, Bitboard::EMPTY), Bitboard::from_squares(&[E3, E4]));
        assert_eq!(get_wpawn_squares(E3, Bitboard::EMPTY), Bitboard::from_squares(&[E4]));
        assert_eq!(get_wpawn_squares(E7, Bitboard::EMPTY), Bitboard::from_squares(&[E8]));
    }

    #[test]
    fn black_pushes() {
        assert_eq!(get_bpawn_squares(E7, Bitboard::EMPTY), Bitboard::from_squares(&[E6, E5]));
        assert_eq!(get_bpawn_squares(E6, Bitboard::EMPTY), Bitboard::from_squares(&[E5]));
        assert_eq!(get_bpawn_squares(E2, Bitboard::EMPTY), Bitboard::from_squares(&[E1]));
    }

    #[test]
    fn blocked_pushes() {
        assert_eq!(get_wpawn_squares(E2, Bitboard::from_squares(&[E3])), Bitboard::EMPTY);
        assert_eq!(get_wpawn_squares(E2, Bitboard::from_squares(&[E4])), Bitboard::from_squares(&[E3]));
        assert_eq!(get_bpawn_squares(E7, Bitboard::from_squares(&[E6])), Bitboard::EMPTY);
        assert_eq!(get_bpawn_squares(E7, Bitboard::from_squares(&[E5])), Bitboard::from_squares(&[E6]));
    }

    #[test]
    fn captures() {
        let occupancy = Bitboard::from_squares(&[D3, F3, D5, F5]);
        assert_eq!(get_wpawn_squares(E4, occupancy), Bitboard::from_squares(&[E5, D5, F5]));
        assert_eq!(get_bpawn_squares(E4, occupancy), Bitboard::from_squares(&[E3, D3, F3]));
        assert_eq!(get_wpawn_squares(A2, Bitboard::from_squares(&[B3, H3])), Bitboard::from_squares(&[A3, A4, B3]));
    }

    #[test]
    fn dispatch_by_color() {
        let occupancy = Bitboard::from_squares(&[D5]);
        assert_eq!(get_pawn_squares(E4, Color::White, occupancy), get_wpawn_squares(E4, occupancy));
        assert_eq!(get_pawn_squares(E4, Color::Black, occupancy), get_bpawn_squares(E4, occupancy));
    }

    #[test]
    fn attacks() {
        assert_eq!(get_pawn_attacks(E4, Color::White), Bitboard::from_squares(&[D5, F5]));
        assert_eq!(get_pawn_attacks(E4, Color::Black), Bitboard::from_squares(&[D3, F3]));
        assert_eq!(get_pawn_attacks(A2, Color::White), Bitboard::from_squares(&[B3]));
        assert_eq!(get_pawn_attacks(H7, Color::Black), Bitboard::from_squares(&[G6]));
        assert_eq!(get_pawn_attacks(E8, Color::White), Bitboard::EMPTY);
        assert_eq!(get_pawn_attacks(E1, Color::Black), Bitboard::EMPTY);
    }

    #[test]
    fn attacks_ignore_occupancy() {
        let occupancy = Bitboard::from_squares(&[E5]);
        let captures = get_wpawn_squares(E4, occupancy).without(Bitboard::from_squares(&[E5]));
        assert_eq!(captures, Bitboard::EMPTY);
        assert_eq!(get_pawn_attacks(E4, Color::White), Bitboard::from_squares(&[D5, F5]));
    }
}
