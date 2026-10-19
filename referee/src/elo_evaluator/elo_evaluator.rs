use super::stockfish_handler::StockfishHandler;
use crate::{config::BotConfig};
use bot::allocators::allocator::IAllocator;
use bot::bot::bot::IBot;
use engine::game::game::{Game, GameEnum};
use std::{matches, println};

enum GameResult {Win, Draw, Loss} // from our bot perspective
const STOCKFISH_MIN_ELO: f64 = 1320.0; 
const STOCKFISH_MAX_ELO: f64 = 3190.0;

pub struct EloEvaluator<A: IAllocator, B: IBot> {
    bot_config: BotConfig<A, B>,
    stockfish_handler: StockfishHandler,
    game: Game,
}

impl<A: IAllocator, B: IBot> EloEvaluator<A, B> {
    pub fn new(bot_config: BotConfig<A, B>) -> Self {
        Self {
            bot_config,
            stockfish_handler: StockfishHandler::new().unwrap(),
            game: Game::new_with_random_opening(4),
        }
    } 

    pub fn evaluate(&mut self) -> u32 {
        let mut stockfish_elo = STOCKFISH_MIN_ELO;
        let mut estimates = Vec::new();
        self.stockfish_handler.set_elo(STOCKFISH_MIN_ELO as u32).unwrap();

        for block in 0..20 {
            let mut sum_score = 0.0;
            for i in 0..20 {
                let our_bot_is_white = i & 1 == 0;
                sum_score += match self.play_a_game(our_bot_is_white) {
                    GameResult::Win => 1.0, 
                    GameResult::Loss => 0.0, 
                    GameResult::Draw => 0.5,
                };

            }

            let est = Self::estimate(stockfish_elo, sum_score / 20.0);
            if block >= 4 {
                estimates.push(est);
            }

            stockfish_elo = est.clamp(STOCKFISH_MIN_ELO, STOCKFISH_MAX_ELO).round();
            self.stockfish_handler.set_elo(stockfish_elo as u32).unwrap();

            println!("block {block}, elo: {est}")
        }

        (estimates.iter().sum::<f64>() / estimates.len() as f64) as u32
    }

    fn estimate(stockfish_elo: f64, score: f64) -> f64 {
        let score = score.clamp(0.05, 0.95);
        stockfish_elo + 400.0 * (score / (1.0 - score)).log10()
    }

    fn play_a_game(&mut self, our_bot_is_white: bool) -> GameResult {
        self.game = Game::new();
        self.stockfish_handler.new_game().unwrap();

        let mut uci_moves: Vec<String> = Vec::new();
        let mut bot_turn = our_bot_is_white;

        while matches!(self.game.game_enum, GameEnum::InAction) {
            let piece_move = if bot_turn {
                self.bot_config.bot.get_best_move(&self.game)
            } else {
                let uci = self.stockfish_handler.get_move(&uci_moves).unwrap();
                self.game.uci_to_piece_move(&uci)
            };

            uci_moves.push(self.game.piece_move_to_uci(&piece_move));
            self.game.do_move(&piece_move);
            self.game.try_update_game_enum();
            bot_turn = !bot_turn;
        }

        match (&self.game.game_enum, our_bot_is_white) {
            (GameEnum::WhiteWon, true) | (GameEnum::BlackWon, false) => GameResult::Win,
            (GameEnum::WhiteWon, false) | (GameEnum::BlackWon, true) => GameResult::Loss,
            (_, _) => GameResult::Draw,
        }
    }
}