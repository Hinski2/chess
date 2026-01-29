use chess::app::App;
use chess::PlayerType;
use bot::bot::random_bot::RandomBot;
use bot::bot::mc_bot::McBot;
use bot::bot::min_max::MinMaxBot;
use bot::allocators::list_stack_allocator::ListStackAllocator;

fn main() {
    // white
    let allocator = ListStackAllocator::new();
    let white_bot = MinMaxBot::new(allocator, 5);
    let white_player = PlayerType::Bot(Box::new(white_bot));

    // black
    let allocator = ListStackAllocator::new();
    let black_bot = MinMaxBot::new(allocator, 5);
    let black_player = PlayerType::Bot(Box::new(black_bot));

    // run the game
    let mut app = App::new(white_player, black_player);
    app.run();
}
