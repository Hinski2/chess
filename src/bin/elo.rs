use bot::allocators::list_stack_allocator::ListStackAllocator;
use bot::bot::min_max::MinMaxBot;
use referee::elo_evaluator::elo_evaluator::EloEvaluator;
use eval::static_evaluation::static_evaluation::StaticEvaluator;

fn main() {
    let eval = StaticEvaluator;
    let bot = MinMaxBot::new(ListStackAllocator::new(), eval, 3);

    let mut elo_evaluator = EloEvaluator::new(bot);
    println!("estimated elo: {}", elo_evaluator.evaluate());
}
