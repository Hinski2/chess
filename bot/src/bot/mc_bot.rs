use engine::{board::{piece_move::PieceMove, Color}, game::game::Game};
use eval::evaluator::IEvaluator;
use crate::{allocators::{allocator::IAllocator, list_stack_allocator::ListStackAllocator, node::INode}, bot::bot::IBot};

pub struct McBot<E: IEvaluator> {
    eval: E,
    memory: ListStackAllocator,
}

impl<E: IEvaluator> McBot<E> {
    pub fn new(eval: E) -> Self {
        Self {
            eval: eval,
            memory: ListStackAllocator::new(),
        }
    }
}

impl<E: IEvaluator> IBot for McBot<E> {
    fn get_best_move(&mut self, game: &Game) -> PieceMove {
        let mut game = Game::from(game);
        let root = self.memory.get_node(0);
        root.clear();
        root.set_moves(game.board.generate_all_moves());

        let mut best_move = root.moves[0].clone();
        let mut best_score = if game.board.get_size_to_move() == Color::White { i32::MIN } else { i32::MAX };
        
        if cfg!(debug_assertions) {
            assert!(matches!(game.game_enum, engine::game::game::GameEnum::InAction));
        }

        for mv in &root.moves {
            game.do_move(&mv);

            let curr_score = self.eval.evaluate(&game);
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
