/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use crate::model::*;
use getset::CopyGetters;
use arrayvec::ArrayVec;
use Piece::*;

pub type MoveList<const N: usize> =
    ArrayVec<Move, N>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Move {
    side_effect: (Square, Square),
    promotion: Option<Piece>,
    landing: Square,
    start: Square,
}

impl Move {
    pub fn new(start: Square, landing: Square, promotion: Option<Piece>, side_effect: (Square, Square)) -> Move {
        Self { start, landing, side_effect, promotion }
    }

    pub fn from_squares(start: Square, landing: Square) -> Move {
        Self { start, landing, side_effect: (start, landing), promotion: None }
    }

    pub fn from_mask<const N: usize>(start: Square, mask: Bitboard) -> MoveList<N> {
        let mapping = |landing| Self::from_squares(start, landing);
        Self::iterate_bitboard(mask).map(mapping).collect()
    }

    pub fn promotions(&self, color : Color) -> MoveList<4> {
        let promote = |p| Move { promotion: Some(p), ..*self };
        let pieces = match color {
            Color::White => [WBishop, WKnight, WRook, WQueen],
            Color::Black => [BBishop, BKnight, BRook, BQueen],
        };
        pieces.into_iter().map(promote).collect()
    }

    fn iterate_bitboard(bitboard : Bitboard) -> impl Iterator<Item = Square> {
        let mut bits = bitboard.bits();
        std::iter::from_fn(move || {
            let index = bits.trailing_zeros() as u8;
            let out = (bits != 0).then(|| {Square::from_index(index)});
            bits &= bits.wrapping_sub(1);
            out
        })
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
    fn new() {
        let castle = Move::new(E1, G1, None, (H1, F1));
        assert_eq!((castle.start, castle.landing, castle.promotion), (E1, G1, None));
        assert_eq!(castle.side_effect, (H1, F1));
        let promo = Move::new(B7, A8, Some(WQueen), (B7, A8));
        assert_eq!(promo.promotion, Some(WQueen));
        assert_eq!(Move::new(E2, E4, None, (E2, E4)), Move::from_squares(E2, E4));
    }

    #[test]
    fn from_squares() {
        let mv = Move::from_squares(E2, E4);
        assert_eq!((mv.start, mv.landing, mv.promotion), (E2, E4, None));
        assert_eq!(mv.side_effect, (E2, E4));
    }

    #[test]
    fn iterate_bitboard() {
        let squares: Vec<Square> = Move::iterate_bitboard(Bitboard::from_squares(&[A1, E4, H8])).collect();
        assert_eq!(squares, vec![H8, E4, A1]);
        assert_eq!(Move::iterate_bitboard(Bitboard::EMPTY).count(), 0);
        assert_eq!(Move::iterate_bitboard(Bitboard::from_bits(u64::MAX)).count(), 64);
    }

    #[test]
    fn from_mask() {
        let moves: MoveList<8> = Move::from_mask(D4, Bitboard::from_squares(&[D5, C3]));
        assert_eq!(moves.as_slice(), &[Move::from_squares(D4, D5), Move::from_squares(D4, C3)]);
        let empty: MoveList<8> = Move::from_mask(D4, Bitboard::EMPTY);
        assert!(empty.is_empty());
    }

    #[test]
    fn promotions() {
        let push = Move::from_squares(E7, E8);
        let promoted: Vec<Option<Piece>> = push.promotions(Color::White).iter().map(|m| m.promotion).collect();
        assert_eq!(promoted, vec![Some(WBishop), Some(WKnight), Some(WRook), Some(WQueen)]);
        for mv in Move::from_squares(D2, C1).promotions(Color::Black) {
            assert_eq!((mv.start, mv.landing), (D2, C1));
            assert_eq!(mv.promotion.map(Piece::color), Some(Color::Black));
        }
    }
}
