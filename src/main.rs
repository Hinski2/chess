use bot::bot::bot::IBot;
use bot::move_order::no_order::NoOrder;
use chess::app::App;
use chess::PlayerType;
use bot::bot::mc_bot::McBot;
use bot::bot::min_max::MinMaxBot;
use bot::allocators::list_stack_allocator::ListStackAllocator;
use eval::static_evaluation::static_evaluation::StaticEvaluator;
use eval::random_evaluation::monte_carlo::MonteCarloEvaluator;

fn main() {
    // white
    let allocator = ListStackAllocator::new();
    let eval = StaticEvaluator;
    let move_order = NoOrder::new();
    let white_bot: Box<dyn IBot> = Box::new(MinMaxBot::new(eval,  allocator, move_order, 3));
    let white_player = PlayerType::Bot(white_bot);

    // black
    let eval = MonteCarloEvaluator::new(10);
    let allocator = ListStackAllocator::new();
    let move_order = NoOrder::new();
    let black_bot: Box<dyn IBot> = Box::new(McBot::new(eval, allocator, move_order));
    let black_player = PlayerType::Bot(black_bot);

    // run the game
    let mut app = App::new(white_player, black_player);
    app.run();
}
