use engine::{board::{piece_move::PieceMove, Color}, game::game::Game};
use eval::evaluator::IEvaluator;
use crate::{allocators::allocator::IAllocator, bot::bot::IBot, move_order::move_order::IMoveOrder, node::{node::INode, node_min_max::NodeMinMax}};

pub struct McBot<E: IEvaluator, A: IAllocator<NodeMinMax>, O: IMoveOrder> {
    eval: E,
    memory: A,
    move_order: O,
}

impl<E: IEvaluator, A: IAllocator<NodeMinMax>, O: IMoveOrder> McBot<E, A, O> {
    pub fn new(eval: E, memory: A, move_order: O) -> Self {
        Self {
            eval, 
            memory, 
            move_order,
        }
    }
}

impl<E: IEvaluator, A: IAllocator<NodeMinMax>, O: IMoveOrder> IBot for McBot<E, A, O> {
    fn get_best_move(&mut self, game: &Game) -> PieceMove {
        let mut game = Game::from(game);
        assert!(matches!(game.game_enum, engine::game::game::GameEnum::InAction));

        self.memory.new_search();
        self.move_order.new_search();

        let root = self.memory.get_root_id(&game);
        let no_moves = self.memory.get_node(root).get_move_list().prepare_moves(&mut self.move_order, &mut game, 0);
        assert!(no_moves > 0);

        let maximizing = game.board.get_size_to_move() == Color::White;
        let mut best_move_idx = 0;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

        for i in 0..no_moves {
            game.do_move(self.memory.get_node(root).get_move_list().get_ith_move(i));
            let score = self.eval.evaluate(&game);
            game.undo_move();

            if (maximizing && score > best_score) || (!maximizing && score < best_score) {
                best_score = score;
                best_move_idx = i;
            }
        }

        self.memory.get_node(root).get_move_list().get_ith_move(best_move_idx).clone()
    }
}
