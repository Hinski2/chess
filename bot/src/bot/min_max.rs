use std::{debug_assert_eq, matches};

use engine::{board::{Color, piece_move::PieceMove}, game::game::{Game, GameEnum}};
use eval::{evaluator::IEvaluator, static_evaluation::MATE_VALUE};

use crate::{allocators::allocator::IAllocator, bot::bot::IBot, move_order::move_order::IMoveOrder, node::{node::INode, node_min_max::NodeMinMax}};

pub struct MinMaxBot<E: IEvaluator, A: IAllocator<NodeMinMax>, O: IMoveOrder>
{
    eval: E,
    memory: A,
    move_order: O,
    max_deep: usize,
}

impl<E: IEvaluator, A: IAllocator<NodeMinMax>, O: IMoveOrder> MinMaxBot<E, A, O>
{
    pub fn new(eval: E, memory: A, move_order: O, max_deep: usize) -> Self {
        Self { eval, memory, move_order, max_deep }
    }

    fn do_min_max(&mut self, game: &mut Game, node: A::NodeId, deep: usize) -> i32 {
        debug_assert_eq!(game.get_states_stack_size(), deep);

        if !matches!(game.game_enum, GameEnum::InAction) {
            return 0;
        }

        if deep == self.max_deep {
            return self.eval.evaluate(game);
        }

        let no_moves = self
            .memory
            .get_node(node)
            .get_move_list()
            .prepare_moves(&mut self.move_order, game, deep);

        if no_moves == 0 {
            return Self::get_terminal_score(game, deep);
        }

        let maximizing = game.board.get_size_to_move() == Color::White;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

        for i in 0..no_moves {
            game.do_move(self.memory.get_node(node).get_move_list().get_ith_move(i));
            let child = self.memory.get_ith_child_id(node, i, game);
            let score = self.do_min_max(game, child, deep + 1);
            game.undo_move();

            best_score = if maximizing { best_score.max(score) } else { best_score.min(score) };
        }

        best_score
    }

    fn get_terminal_score(game: &mut Game, deep: usize) -> i32 {
       if !game.board.is_checked() {
        return 0;
       } 

       let mate = MATE_VALUE - deep as i32;
       if game.board.get_size_to_move() == Color::White { -mate } else { mate }
    }
}

impl<E: IEvaluator, A: IAllocator<NodeMinMax>, O: IMoveOrder> IBot for MinMaxBot<E, A, O>
{
    fn get_best_move(&mut self, game: &Game) -> PieceMove {
        let mut game = Game::from(game);
        assert!(matches!(game.game_enum, GameEnum::InAction));

        self.memory.new_search();
        self.move_order.new_search();

        let root = self.memory.get_root_id(&game);
        let no_moves = self
            .memory
            .get_node(root)
            .get_move_list()
            .prepare_moves(&mut self.move_order, &mut game, 0);
        assert!(no_moves > 0);

        let maximizing = game.board.get_size_to_move() == Color::White;
        let mut best_i = 0;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

        for i in 0..no_moves {
            game.do_move(self.memory.get_node(root).get_move_list().get_ith_move(i));
            let child = self.memory.get_ith_child_id(root, i, &game);
            let score = self.do_min_max(&mut game, child, 1);
            game.undo_move();

            if (maximizing && score > best_score) || (!maximizing && score < best_score) {
                best_score = score;
                best_i = i;
            }
        }
        
        self.memory.get_node(root).get_move_list().get_ith_move(best_i).clone()
    }
}
