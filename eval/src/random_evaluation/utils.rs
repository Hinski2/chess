use engine::game::game::{Game, GameEnum};
use rand::{seq::IndexedRandom};

pub(super) fn random_walk(game: &mut Game) -> i32 {
    let mut rng = rand::rng();
    let mut limit = 100;

    while let GameEnum::InAction = game.game_enum {
        if limit == 0 {
            break;
        } else {
            limit -= 1;
        }

        let moves = game.board.generate_all_moves();
        if moves.is_empty() {
            game.check_for_mate_or_stalemate();
            break;
        }
        
        let random_move = moves.choose(&mut rng).unwrap();
        game.do_move(random_move);
    }

    match game.game_enum {
        GameEnum::WhiteWon => 1,
        GameEnum::BlackWon => -1,
        _ => 0,
    }
}