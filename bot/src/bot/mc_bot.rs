use engine::{board::{piece_move::PieceMove, Color}, game::game::Game, utils::evaluation_function::monte_carlo};
use crate::{allocators::{allocator::IAllocator, list_stack_allocator::ListStackAllocator, node::INode}, bot::bot::IBot};

pub struct McBot {
    memory: ListStackAllocator,
    no_itr: usize,
}

impl McBot {
    pub fn new(no_itr: usize) -> Self {
        Self {
            memory: ListStackAllocator::new(),
            no_itr: no_itr,
        }
    }
}

impl IBot for McBot {
    fn get_best_move(&mut self, game: &Game) -> PieceMove {
        let mut game = Game::from(game);
        let root = self.memory.get_node(0);
        root.clear();
        root.set_moves(game.board.generate_all_moves());

        let mut best_move = root.moves[0].clone();
        let mut best_score = if game.board.get_size_to_move() == Color::White { i32::MIN } else { i32::max_value() };
        

        for mv in &root.moves {
            game.do_move(&mv);
            if cfg!(debug_assertions) {
                assert!(matches!(game.game_enum, engine::game::game::GameEnum::InAction));
            }

            let curr_score = monte_carlo(&game, self.no_itr);
            game.undo_move();

            if curr_score > best_score && game.board.get_size_to_move() == Color::White {
                best_score = curr_score;
                best_move = mv.clone();
            } else if curr_score < best_score && game.board.get_size_to_move() == Color::Black {
                best_score = curr_score;
                best_move = mv.clone();
            }
        }
    
        best_move
    }
}
