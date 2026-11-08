use engine::{board::Color, game::game::{Game, GameEnum}};
use eval::{evaluator::IEvaluator, static_evaluation::MATE_VALUE};

use crate::{allocators::allocator::IAllocator, bot::{INF, bot::IBot}, move_order::move_order::IMoveOrder, node::{node::INode, node_alfa_beta::NodeAlfaBeta}};

pub struct AlfaBetaBot<E: IEvaluator, A: IAllocator<NodeAlfaBeta>, O: IMoveOrder> {
    eval: E, 
    memory: A, 
    move_order: O, 
    max_deep: usize
}

impl <E: IEvaluator, A: IAllocator<NodeAlfaBeta>, O: IMoveOrder> AlfaBetaBot<E, A, O> {
    pub fn new(eval: E, memory: A, move_order: O, max_deep: usize) -> Self {
        Self { eval, memory, move_order, max_deep }
    }

    fn do_alfa_beta(&mut self, game: &mut Game, node: A::NodeId, deep: usize, mut alfa: i32, mut beta: i32) -> i32 {
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
        let mut best_score = if maximizing { -INF } else { INF };

        for i in 0..no_moves {
            game.do_move(self.memory.get_node(node).get_move_list().get_ith_move(i));
            let child = self.memory.get_ith_child_id(node, i, game);
            let score = self.do_alfa_beta(game, child, deep + 1, alfa, beta);
            game.undo_move();

            if maximizing {
                best_score = best_score.max(score);
                alfa = alfa.max(score);
            } else {
                best_score = best_score.min(score);
                beta = beta.min(score);
            }

            if alfa >= beta {
                self.move_order.on_cutoff(deep, self.memory.get_node(node).get_move_list().get_ith_move(i));
                break;
            }
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

impl<E: IEvaluator, A: IAllocator<NodeAlfaBeta>, O: IMoveOrder> IBot for AlfaBetaBot<E, A, O> {
    fn get_best_move(&mut self, game: &Game) -> engine::board::piece_move::PieceMove {
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
        let mut alfa = -INF; 
        let mut beta = INF; 

        for i in 0..no_moves {
            game.do_move(self.memory.get_node(root).get_move_list().get_ith_move(i));
            let child = self.memory.get_ith_child_id(root, i, &game);
            let score = self.do_alfa_beta(&mut game, child, 1, alfa, beta);
            game.undo_move();

            if (maximizing && score > best_score) || (!maximizing && score < best_score) {
                best_score = score;
                best_i = i;
            }

            if maximizing { alfa = alfa.max(score) } else { beta = beta.min(score) };
        }
        
        self.memory.get_node(root).get_move_list().get_ith_move(best_i).clone()
    }
}