use chess::app::App;
use chess::PlayerType;
use bot::bot::random_bot::RandomBot;

fn main() {
    // init 
    let white_bot = RandomBot::new();
    let white_player = PlayerType::Bot(Box::new(white_bot));

    let black_bot = RandomBot::new();
    let black_player = PlayerType::Bot(Box::new(black_bot));

    let mut app = App::new(white_player, black_player);
    app.run();
}
