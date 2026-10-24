use engine::game::game::Game;
pub trait IEvaluator {
    fn evaluate(&self, game: &Game) -> i32;
}