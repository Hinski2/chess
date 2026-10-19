use std::collections::{HashMap};
use rand::seq::IndexedRandom;

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
        let board = Board::new();
        let mut hshs = HashMap::new();
        hshs.insert(board.get_board_hsh(), 1);

        Game { board,
               states: Vec::new(),
               hshs,
               game_enum: GameEnum::InAction,
        }
    } 

    pub fn new_with_random_opening(random_moves: usize) -> Game {
        let board = Board::new();
        let mut hshs = HashMap::new();
        hshs.insert(board.get_board_hsh(), 1);

        let mut game = Game { board,
               states: Vec::new(),
               hshs,
               game_enum: GameEnum::InAction,
        };

        let mut rng = rand::rng();
        for _ in 0..random_moves {
            let valid_moves = game.board.generate_all_moves();
            let random_move = valid_moves.choose(&mut rng).unwrap();
            game.do_move(random_move);
            game.try_update_game_enum();
        }

        game
    }

    pub fn piece_move_to_uci(&self, pm: &PieceMove) -> String {
        let from = Self::square_to_uci(pm.from);
        let to = Self::square_to_uci(pm.to);

        let mut uci = format!("{from}{to}");
        if let Some(promotion) = Self::promotion_piece_char(&pm.flag) {
            uci.push(promotion);
        }

        uci
    }

    pub fn uci_to_piece_move(&self, uci: &str) -> PieceMove {
        let chars: Vec<char> = uci.trim().to_ascii_lowercase().chars().collect();

        if chars.len() != 4 && chars.len() != 5 {
            panic!("invalid UCI move: {uci}");
        }

        let from = Self::uci_to_square(chars[0], chars[1]);
        let to = Self::uci_to_square(chars[2], chars[3]);
        let promotion = chars.get(4).copied();

        let mut board = self.board.clone();
        board.generate_all_moves()
            .into_iter()
            .find(|pm| {
                pm.from == from
                && pm.to == to
                && Self::promotion_piece_char(&pm.flag) == promotion
            }).unwrap_or_else(| | panic!("Invalid UCI move: {uci}"))
    }

    fn square_to_uci(square: u8) -> String {
        let file = (b'a' + square % 8) as char;
        let rank = (b'1' + square / 8) as char;
        return format!("{file}{rank}");
    }

    fn uci_to_square(file: char, rank: char) -> u8 {
        (rank as u8 - '1' as u8) * 8 + (file as u8 - 'a' as u8)
    }

    fn promotion_piece_char(flag: &MoveFlag) -> Option<char> {
        match flag {
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToQueenAndCapture => Some('q'),
            MoveFlag::PromoteToRook | MoveFlag::PromoteToRookAndCapture => Some('r'),
            MoveFlag::PromoteToBishop | MoveFlag::PromoteToBishopAndCapture => Some('b'),
            MoveFlag::PromoteToKnight | MoveFlag::PromoteToKnightAndCapture => Some('n'),
            _ => None,
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
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToRook |
            MoveFlag::PromoteToBishop | MoveFlag::PromoteToKnight |
            MoveFlag::DoublePawnPush => 0,
            MoveFlag::Normal => {
                if self.board.pieces[piece_move.from as usize].extract_piece() == Piece::Pawn { 0 } else {new_half_move_clock + 1}
            },
            _ => new_half_move_clock + 1,
        };

        // save old state
        let captured_piece = self.board.pieces[piece_move.to as usize].try_extract_piece();
        let old_board_state = self.board.get_board_state();

        self.states.push( GameState { piece_move: piece_move.clone(), board_state: old_board_state, half_move_clock: new_half_move_clock, captured_piece});
        self.board.do_move(&piece_move);

        // check for draws on the new position
        let new_hsh = self.board.get_board_hsh();
        *self.hshs.entry(new_hsh).or_insert(0) += 1;
        self.check_for_draws(new_hsh, new_half_move_clock);
    }

    /// undo last move
    pub fn undo_move(&mut self) {
        if cfg!(debug_assertions) {
            assert!(!self.states.is_empty());
        }

        self.game_enum = GameEnum::InAction; // because we could make the next move
        let hsh = self.board.get_board_hsh();
        self.hshs.entry(hsh)
            .and_modify(| e| *e -= 1);

        let piece_move = &self.states.last().unwrap().piece_move; 
        let board_state = &self.states.last().unwrap().board_state;
        let captured_piece = self.states.last().unwrap().captured_piece;

        self.board.undo_move(&piece_move, board_state.clone(), captured_piece);
        self.states.pop();
    }
}
