use std::matches;

pub(crate) use crate::allocators::allocator::IAllocator;
use crate::allocators::node::INode;
use engine::{board::{Color, piece_move::{PieceMove}}, game::game::{Game, GameEnum}};
use super::bot::IBot;
use eval::evaluator::IEvaluator;
use eval::static_evaluation::MATE_VALUE;

pub struct MinMaxBot<A: IAllocator, E: IEvaluator>
{
    memory: A,
    eval: E,
    max_deep: usize,
}

impl<A: IAllocator<Key = usize>, E: IEvaluator> MinMaxBot<A, E>
{
    pub fn new(allocator: A, eval: E, max_deep: usize) -> Self {
        Self {
            memory: allocator,
            eval: eval,
            max_deep: max_deep,
        }
    }

    fn do_min_max(&mut self, mv: &PieceMove, deep: usize, game: &mut Game) -> i32 {
        game.do_move(mv);  // check for draws is checked in this function

        if cfg!(debug_assertions) {
            assert!(deep == game.get_states_stack_size());
        }

        if !matches!(game.game_enum, GameEnum::InAction) {
            game.undo_move();
            return 0;
        }

        if deep == self.max_deep {
            let score = self.eval.evaluate(game);
            game.undo_move();
            return score;
        }

        self.memory.get_node(deep).clear();
        let mut moves = self.memory.get_node(deep).take_moves();
        game.board.generate_all_moves_into(&mut moves);

        if moves.is_empty() {
            game.try_update_game_enum();
            let score = match game.game_enum {
                GameEnum::WhiteWon => MATE_VALUE,
                GameEnum::BlackWon => -MATE_VALUE,
                _ => 0, // it must be a draw
            };

            self.memory.get_node(deep).set_moves(moves);
            game.undo_move();
            return score;
        }

        let maximizing = game.board.get_size_to_move() == Color::White; 
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

        for mv in &moves {
            let score = self.do_min_max(mv, deep + 1, game);
            best_score = if maximizing { best_score.max(score) } else { best_score.min(score) }
        }

        self.memory.get_node(deep).set_moves(moves);
        game.undo_move();
        best_score
    }
}

impl<A: IAllocator<Key = usize>, E: IEvaluator> IBot for MinMaxBot<A, E>
{
    fn get_best_move(&mut self, game: &Game) -> PieceMove {
        let mut game = Game::from(game);

        let root_node = self.memory.get_node(0);
        root_node.clear();
        let moves = game.board.generate_all_moves();

        let mut best_move = moves[0].clone();
        let mut best_score = if game.board.get_size_to_move() == Color::White { i32::MIN } else { i32::MAX };

        // white -> try to max 
        // black -> try to min

        for mv in moves {
            if cfg!(debug_assertions) {
                assert!(game.get_states_stack_size() == 0);
            }

            let score = self.do_min_max(&mv, 1, &mut game);

            if score > best_score && game.board.get_size_to_move() == Color::White {
                best_score = score;
                best_move = mv.clone();
            } else if score < best_score && game.board.get_size_to_move() == Color::Black {
                best_score = score;
                best_move = mv.clone();
            }
        }
    
        best_move
    }
}
