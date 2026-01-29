use std::collections::{HashMap};
use crate::board::board::Board;
use crate::board::{BoardState, Piece};
use crate::board::piece_move::{MoveFlag, PieceMove};

#[derive(Debug)]
pub(crate) struct GameState {
    piece_move: PieceMove,
    board_state: BoardState,
    half_move_clock: usize, // used for 50 move rule
    pub captured_piece: Option<Piece>,
}

#[derive(Clone, Debug)]
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
    pub board: Board,
    pub(crate) states: Vec<GameState>,
    pub(crate) hshs: HashMap<u64, usize>,
    pub game_enum: GameEnum,
}

impl Game {
    pub fn new() -> Game {
        Game { board: Board::new(),
               states: Vec::new(),
               hshs: HashMap::new(),
               game_enum: GameEnum::InAction,
        }
    } 

    pub fn print_states_stack(&self) {
        println!("&{:?}", &self.states);
    }

    pub fn get_states_stack_size(&self) -> usize {
        self.states.len()
    }

    pub fn from(game: &Game) -> Game {
        Game {
            board: game.board.clone(),
            states: Vec::new(), 
            hshs: game.hshs.clone(), 
            game_enum: game.game_enum.clone(),
        }
    }
    
    /// updates game_enum if it's game over
    pub fn try_update_game_enum(&mut self) {

        let moves = self.board.generate_all_moves();
        if moves.is_empty() {
            self.check_for_mate_or_stalemate();
        }
    }
    
    /// makes the move and update gameEnum for draws,
    /// you have to handle gameEnum for end separately
    pub fn do_move(&mut self, piece_move: &PieceMove) {
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
        let captured_piece = self.board.pieces[piece_move.to as usize].try_extract_piece();
        let (old_board_state, old_hsh) = (self.board.get_board_state(), self.board.get_board_hsh());

        self.states.push( GameState { piece_move: piece_move.clone(), board_state: old_board_state, half_move_clock: new_half_move_clock, captured_piece});
        *self.hshs.entry(old_hsh).or_insert(0) += 1;

        self.check_for_draws(old_hsh, new_half_move_clock); 
        // if !matches!(self.game_enum, GameEnum::InAction) {
            // return;
        // }

        // everything ok
        self.board.do_move(&piece_move);
    }

    /// undo last move
    pub fn undo_move(&mut self) {
        if cfg!(debug_assertions) {
            assert!(!self.states.is_empty());
        }

        self.game_enum = GameEnum::InAction; // because we could make the next move
        let piece_move = &self.states.last().unwrap().piece_move; 
        let board_state = &self.states.last().unwrap().board_state;
        let captured_piece = self.states.last().unwrap().captured_piece;

        self.board.undo_move(&piece_move, board_state.clone(), captured_piece);
        self.states.pop();

        let hsh = self.board.get_board_hsh();
        self.hshs.entry(hsh)
            .and_modify(| e| *e -= 1);
    }
}
