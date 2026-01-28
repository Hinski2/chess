use chess::app::App;
use chess::PlayerType;
use bot::bot::random_bot::RandomBot;
use bot::bot::mc_bot::McBot;
use bot::bot::min_max::MinMaxBot;
use bot::allocators::list_stack_allocator::ListStackAllocator;

fn main() {
    // white
    let white_bot = MinMaxBot::new(ListStackAllocator::new(), 2);
    let white_player = PlayerType::Bot(Box::new(white_bot));

    // black
    let black_bot = RandomBot::new();
    let black_player = PlayerType::Bot(Box::new(black_bot));

    // run the game
    let mut app = App::new(white_player, black_player);
    app.run();
}
