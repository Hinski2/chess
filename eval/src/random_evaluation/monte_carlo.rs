use engine::game::game::Game;
use super::utils::random_walk;

pub fn monte_carlo(game: &Game, itr: usize) -> i32 {
    let mut score = 0;

    for _ in 0..itr {
        let mut game_simulation = Game::from(game);
        score += random_walk(&mut game_simulation);
    }

    score
}