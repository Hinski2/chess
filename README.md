# Chess 

--- 

### how to install 

Just type `cargo run` if you want to use crate with tests or `cargo run --release` for better performance


### how to use

go to src/main.rs:
``` rust
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
```

you can set here which bot you want to use:
* random bot: `RandomBot::new()`
* mc_bot: `McBot::new(itr)` where itr is the noumber of iterations for each son
* min_max: `MinMaxBot::new(allocator, d)` where d is the max depth in MinMax, and the only available allocator for now is ListStackAllocator


