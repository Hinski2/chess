//! note we retrun score from white perspective
use crate::{
    board::{Color, Piece},
    game::game::{Game, GameEnum},
};
use rand::{seq::IndexedRandom};

const PAWN_VALUE: i32 = 1;
const KNIGHT_VALUE: i32 = 3;
const BISHOP_VALUE: i32 = 3;
const ROOK_VALUE: i32 = 5;
const QUEEN_VALUE: i32 = 9;
// const KING_VALUE: i32 = 100;

pub const MATE_VALUE: i32 = 1000;

pub fn monte_carlo(game: &Game, itr: usize) -> i32 {
    let mut score = 0;

    for _ in 0..itr {
        let mut game_simulation = Game::from(game);
        score += random_walk(&mut game_simulation);
    }

    score
}

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

    let score = evaluate_pawns(game);
    score
}

fn evaluate_pawns(game: &Game) -> i32 {
    let mut score = 0;

    let w_pawns = game.board.bitboard[Piece::Pawn as usize][Color::White as usize].count_ones() as i32;
    let b_pawns = game.board.bitboard[Piece::Pawn as usize][Color::Black as usize].count_ones() as i32;
    score += (w_pawns - b_pawns) * PAWN_VALUE;

    let w_knights = game.board.bitboard[Piece::Knight as usize][Color::White as usize].count_ones() as i32;
    let b_knights = game.board.bitboard[Piece::Knight as usize][Color::Black as usize].count_ones() as i32;
    score += (w_knights - b_knights) * KNIGHT_VALUE;

    let w_bishops = game.board.bitboard[Piece::Bishop as usize][Color::White as usize].count_ones() as i32;
    let b_bishops = game.board.bitboard[Piece::Bishop as usize][Color::Black as usize].count_ones() as i32;
    score += (w_bishops - b_bishops) * BISHOP_VALUE;

    let w_rooks = game.board.bitboard[Piece::Rook as usize][Color::White as usize].count_ones() as i32;
    let b_rooks = game.board.bitboard[Piece::Rook as usize][Color::Black as usize].count_ones() as i32;
    score += (w_rooks - b_rooks) * ROOK_VALUE;

    let w_queens = game.board.bitboard[Piece::Queen as usize][Color::White as usize].count_ones() as i32;
    let b_queens = game.board.bitboard[Piece::Queen as usize][Color::Black as usize].count_ones() as i32;
    score += (w_queens - b_queens) * QUEEN_VALUE;

    score
}

fn random_walk(game: &mut Game) -> i32 {
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
