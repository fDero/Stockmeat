/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::model::{Board, Color, Game, Rights, Square};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PositionKey {
    en_passant_square: Option<Square>,
    castling_rights: Rights,
    current_turn: Color,
    board: Board,
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

pub struct History {
    reps: HashMap<PositionKey, u8>,
    moves_log: Vec<Game>,
}

impl History {
    pub fn new() -> Self {
        History { reps: HashMap::new(), moves_log: Vec::new() }
    }

    pub fn add(&mut self, game: Game) {
        *self.reps.entry(PositionKey::from(&game)).or_insert(0) += 1;
        self.moves_log.push(game);
    }

    pub fn takeback(&mut self) -> Option<Game> {
        let game = self.moves_log.pop()?;
        let key = PositionKey::from(&game);
        match self.reps.get_mut(&key) {
            Some(count) if *count > 1 => *count -= 1,
            _ => { self.reps.remove(&key); }
        }
        Some(game)
    }

    pub fn latest(&self) -> Option<&Game> {
        self.moves_log.last()
    }

    pub fn count(&self, game: &Game) -> u8 {
        self.reps.get(&PositionKey::from(game)).copied().unwrap_or(0)
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
    use crate::model::Move;
    use Square::*;

    fn reps(history: &History, game: &Game) -> u8 {
        history.count(game)
    }

    fn play(history: &mut History, game: Game, moves: &[(Square, Square)]) -> Game {
        moves.iter().fold(game, |game, &(start, landing)| {
            let next = game.apply_unchecked(Move::from_squares(start, landing));
            history.add(next);
            next
        })
    }

    #[test]
    fn new_history_is_empty() {
        let mut history = History::new();
        assert_eq!(reps(&history, &Game::default()), 0);
        assert!(history.takeback().is_none());
    }

    #[test]
    fn add_then_takeback_returns_the_same_game() {
        let mut history = History::new();
        let start = Game::default();
        let after = start.apply_unchecked(Move::from_squares(E2, E4));
        history.add(start);
        history.add(after);
        assert_eq!(history.latest(), Some(&after));
        assert_eq!(history.takeback(), Some(after));
        assert_eq!(history.latest(), Some(&start));
        assert_eq!(history.takeback(), Some(start));
        assert_eq!(history.latest(), None);
        assert!(history.takeback().is_none());
        assert!(history.reps.is_empty());
    }

    #[test]
    fn takeback_decrements_and_removes() {
        let mut history = History::new();
        let start = Game::default();
        history.add(start);
        play(&mut history, start, &[
            (G1, F3), (G8, F6), (F3, G1), (F6, G8)
        ]);
        assert_eq!(reps(&history, &start), 2);
        history.takeback();
        assert_eq!(reps(&history, &start), 1);
        for _ in 0..4 {
            history.takeback();
        }
        assert_eq!(reps(&history, &start), 0);
        assert!(history.reps.is_empty());
    }
}
