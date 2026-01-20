use std::collections::{HashMap};
use engine::board::Board;
use engine::{BoardState, Piece};
use engine::piece_move::{MoveFlag, PieceMove};

pub(crate) struct GameState {
    piece_move: PieceMove,
    board_state: BoardState,
    half_move_clock: usize, // used for 50 move rule
}

pub enum GameEnum {
    InAction,

    TieByThreefoldRepetition,
    TieBy50Rule,
    TieBySalemate,
    TieByInsufficientMaterial,

    WhiteWon,
    BlackWon,
}

pub struct Game {
    pub(crate) board: Board,
    pub(crate) states: Vec<GameState>,
    pub(crate) hshs: HashMap<u64, usize>,
    pub(crate) game: GameEnum,
}

impl Game {
    pub fn new() -> Game {
        Game { board: Board::new(),
               states: Vec::new(),
               hshs: HashMap::new(),
               game: GameEnum::InAction,
        }
    } 

    pub fn do_move(&mut self, piece_move: PieceMove) {
        // compute new_tour_couter
        let new_half_move_clock = if self.states.is_empty() { 0 } else {
            self.states.last().unwrap().half_move_clock
        };

        let new_half_move_clock = match piece_move.flag {
            MoveFlag::Capture | MoveFlag::EnPassantCapture |
            MoveFlag::PromoteToQueenAndCapture | MoveFlag::PromoteToRookAndCapture | 
            MoveFlag::PromoteToBishopAndCapture | MoveFlag::PromoteToKnightAndCapture |
            MoveFlag::DoublePawnPush => 0,
            MoveFlag::Normal => {
                if self.board.pieces[piece_move.from as usize].extract_piece() == Piece::Pawn { 0 } else {new_half_move_clock + 1}
            },
            _ => new_half_move_clock + 1,
        };

        // save old state and check for draws
        let (old_board_state, old_hsh) = (self.board.get_board_state(), self.board.get_board_hsh());
        self.states.push( GameState { piece_move: piece_move.clone(), board_state: old_board_state, half_move_clock: new_half_move_clock});
        *self.hshs.entry(old_hsh).or_insert(0) += 1;

        self.check_for_draws(old_hsh, new_half_move_clock); 
        if !matches!(self.game, GameEnum::InAction) {
            return;
        }

        // everything ok
        self.board.do_move(&piece_move);
    }

    // undo last move
    pub fn undo_move(&mut self) {
        if cfg!(debug_assertions) {
            assert!(!self.states.is_empty());
        }

        self.game = GameEnum::InAction; // because we could make the next move
        let piece_move = &self.states.last().unwrap().piece_move; 
        let board_state = self.states.last().unwrap().board_state;

        self.board.undo_move(&piece_move, board_state);
        self.states.pop();
    }
}