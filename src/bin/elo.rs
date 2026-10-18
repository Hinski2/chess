use bot::allocators::list_stack_allocator::ListStackAllocator;
use bot::bot::min_max::MinMaxBot;
use referee::config::BotConfig;
use referee::elo_evaluator::elo_evaluator::EloEvaluator;

fn main() {
    let bot = MinMaxBot::new(ListStackAllocator::new(), 2);
    let config = BotConfig::new(ListStackAllocator::new(), bot);

    let mut evaluator = EloEvaluator::new(config);
    println!("estimated elo: {}", evaluator.evaluate());
}
