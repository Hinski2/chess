use std::{thread, time};

use engine::{board::{piece_move::PieceMove, Color, Piece}, game::game::Game};

use crate::PlayerType;

pub struct App {
    pub(crate) white_player: PlayerType, 
    pub(crate) black_player: PlayerType,
    pub(crate) game: Game,
}

impl App {
    pub fn new(white_player: PlayerType, black_player: PlayerType) -> Self {
        Self {
            white_player,
            black_player,
            game: Game::new(),
        } 
    }

    pub fn display(&self) {
        // clear the display
        print!("\x1b[2J");

        // move rustor to the top left corner 
        print!("\x1b[2J\x1b[1;1H");

        println!("{}", self.game.board);

        println!("\n  game status: {:?}", self.game.game_enum);
        println!("  Move: {:?}", self.game.board.get_size_to_move());
    }

    pub fn run(&mut self) {
        self.display();

        while matches!(self.game.game_enum, engine::game::game::GameEnum::InAction) {
            self.make_move();
            self.display();
        }

        self.game.try_update_game_enum();
    }

    fn make_move(&mut self) {
        let game = &self.game;
        // get move
        let chosen_move = match self.game.board.get_size_to_move() {
            Color::White => Self::get_move_from_player(&mut self.white_player, game),
            Color::Black => Self::get_move_from_player(&mut self.black_player, game),
        };
        
        // make move
        self.game.do_move(&chosen_move);
        self.game.try_update_game_enum(); // we have to handle game end check separately
    }

    fn get_move_from_player(player: &mut PlayerType, game: &Game) -> PieceMove {
        match player {
            PlayerType::Human => Self::get_human_move(game),
            PlayerType::Bot(bot) => bot.get_best_move(game)
        } 
    }

    fn get_human_move(game: &Game) -> PieceMove {
        PieceMove { from: 0, to: 0, flag: engine::board::piece_move::MoveFlag::None }
    }
}
