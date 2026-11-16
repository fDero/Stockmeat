/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::model::{Board, Color, Game, Move, Rights, Square};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct History {
    starting_pos: Game,
    snapshots: Vec<(Move, Game)>,
    reps: HashMap<PositionKey, u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PositionKey {
    en_passant_square: Option<Square>,
    castling_rights: Rights,
    current_turn: Color,
    board: Board,
}

impl History {
    pub fn play(&mut self, mov: Move) {
        let game = self.latest().apply_unchecked(mov);
        *self.reps.entry(PositionKey::from(&game)).or_insert(0) += 1;
        self.snapshots.push((mov, game));
    }

    pub fn takeback(&mut self) -> Option<Game> {
        let (_, game) = self.snapshots.pop()?;
        let key = PositionKey::from(&game);
        match self.reps.get_mut(&key) {
            Some(count) if *count > 1 => *count -= 1,
            _ => { self.reps.remove(&key); }
        }
        Some(game)
    }

    pub fn latest(&self) -> Game {
        match self.snapshots.last() {
            Some((_, game)) => *game,
            _ => self.starting_pos,
        }
    }

    pub fn count(&self, game: &Game) -> u8 {
        self.reps.get(&PositionKey::from(game)).copied().unwrap_or(0)
    }
}

impl From<&Game> for PositionKey {
    fn from(game: &Game) -> Self {
        PositionKey {
            en_passant_square: game.en_passant_square,
            castling_rights: game.castling_rights,
            current_turn: game.current_turn,
            board: game.board,
        }
    }
}

impl Hash for PositionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.piece_bitboards().map(|bitboard| bitboard.bits()).hash(state);
        self.board.color_bitboards().map(|bitboard| bitboard.bits()).hash(state);
        (self.current_turn as u8).hash(state);
        self.en_passant_square.map(|square| square as u8).hash(state);
        let rights = self.castling_rights;
        [rights.white_short, rights.white_long, rights.black_short, rights.black_long].hash(state);
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

    fn play(history: &mut History, moves: &[(Square, Square)]) {
        for &(start, landing) in moves {
            history.play(Move::from_squares(start, landing));
        }
    }

    #[test]
    fn new_history_is_empty() {
        let mut history = History::default();
        assert_eq!(history.count(&Game::default()), 0);
        assert_eq!(history.latest(), Game::default());
        assert!(history.takeback().is_none());
    }

    #[test]
    fn play_then_takeback_returns_the_same_game() {
        let mut history = History::default();
        let e4 = Game::default().apply_unchecked(Move::from_squares(E2, E4));
        let e5 = e4.apply_unchecked(Move::from_squares(E7, E5));
        play(&mut history, &[(E2, E4), (E7, E5)]);
        assert_eq!(history.latest(), e5);
        assert_eq!(history.count(&e5), 1);
        assert_eq!(history.takeback(), Some(e5));
        assert_eq!(history.latest(), e4);
        assert_eq!(history.takeback(), Some(e4));
        assert_eq!(history.latest(), Game::default());
        assert!(history.takeback().is_none());
        assert!(history.reps.is_empty());
    }

    #[test]
    fn takeback_decrements_and_removes() {
        let mut history = History::default();
        let shuffle = [(G1, F3), (G8, F6), (F3, G1), (F6, G8)];
        play(&mut history, &shuffle);
        play(&mut history, &shuffle);
        let start = Game::default();
        assert_eq!(history.count(&start), 2);
        history.takeback();
        assert_eq!(history.count(&start), 1);
        for _ in 0..7 {
            history.takeback();
        }
        assert_eq!(history.count(&start), 0);
        assert!(history.reps.is_empty());
    }
}
