use engine::game::game::{Game, GameEnum};
use super::{utils::{evaluate_pawns, evaluate_pos}, MATE_VALUE};

pub fn static_evaluation(game: &Game) -> i32 {
    match game.game_enum {
        GameEnum::WhiteWon => return MATE_VALUE,
        GameEnum::BlackWon => return -MATE_VALUE,
        GameEnum::TieBy50Rule
        | GameEnum::TieByInsufficientMaterial
        | GameEnum::TieBySalemate
        | GameEnum::TieByThreefoldRepetition => return 0,
        GameEnum::InAction => (),
    }

    let mut score = evaluate_pawns(game);
    score += evaluate_pos(game);
    score
}