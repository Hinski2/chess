use engine::game::game::Game;
pub trait IEvalFunction {
    fn evaluate(&mut self, game: &Game) -> i32;
}