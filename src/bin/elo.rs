use bot::allocators::list_stack_allocator::ListStackAllocator;
use bot::bot::min_max::MinMaxBot;
use referee::elo_evaluator::elo_evaluator::EloEvaluator;

fn main() {
    let bot = MinMaxBot::new(ListStackAllocator::new(), 3);

    let mut evaluator = EloEvaluator::new(bot);
    println!("estimated elo: {}", evaluator.evaluate());
}
