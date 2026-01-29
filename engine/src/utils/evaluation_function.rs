//! note we retrun score from white perspective
use crate::{
    board::{Color, Piece},
    game::game::{Game, GameEnum},
};
use rand::{seq::IndexedRandom};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
// const KING_VALUE: i32 = 20000;

pub const MATE_VALUE: i32 = 20000;

// https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_PST: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

const KNIGHT_PST: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_PST: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const ROOK_PST: [i32; 64] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      5, 10, 10, 10, 10, 10, 10,  5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
      0,  0,  0,  5,  5,  0,  0,  0
];

const QUEEN_PST: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const KING_MIDDLE_GAME_PST: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

const KING_END_GAME_PST: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

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

    let mut score = evaluate_pawns(game);
    score += evaluate_pos(game);
    score
}

fn evaluate_bitboard(mut bitboard: u64, pst: &[i32; 64], is_white: bool) -> i32 {
    let mut score: i32 = 0;
    
    while bitboard != 0 {
        let sq = bitboard.trailing_zeros() as usize;
        bitboard &= bitboard - 1;

        if is_white {
            score += pst[sq];
        } else {
            score += pst[sq ^ 56]; 
        }
    }

    score
}

fn evaluate_pos(game: &Game) -> i32 {
    let mut score: i32 = 0;
    
    score += evaluate_bitboard(game.board.bitboard[Piece::Pawn as usize][Color::White as usize], &PAWN_PST, true);
    score -= evaluate_bitboard(game.board.bitboard[Piece::Pawn as usize][Color::Black as usize], &PAWN_PST, false);

    score += evaluate_bitboard(game.board.bitboard[Piece::Knight as usize][Color::White as usize], &KNIGHT_PST, true);
    score -= evaluate_bitboard(game.board.bitboard[Piece::Knight as usize][Color::Black as usize], &KNIGHT_PST, false);

    score += evaluate_bitboard(game.board.bitboard[Piece::Bishop as usize][Color::White as usize], &BISHOP_PST, true);
    score -= evaluate_bitboard(game.board.bitboard[Piece::Bishop as usize][Color::Black as usize], &BISHOP_PST, false);

    score += evaluate_bitboard(game.board.bitboard[Piece::Rook as usize][Color::White as usize], &ROOK_PST, true);
    score -= evaluate_bitboard(game.board.bitboard[Piece::Rook as usize][Color::Black as usize], &ROOK_PST, false);

    score += evaluate_bitboard(game.board.bitboard[Piece::Queen as usize][Color::White as usize ], &QUEEN_PST, true);
    score -= evaluate_bitboard(game.board.bitboard[Piece::Queen as usize][Color::Black as usize], &QUEEN_PST, false);

    let w_queens_count = game.board.bitboard[Piece::Queen as usize][Color::White as usize].count_ones();   
    let b_queens_count = game.board.bitboard[Piece::Queen as usize][Color::Black as usize].count_ones();   
    let king_pst = if w_queens_count == 0 && b_queens_count == 0 { &KING_END_GAME_PST } else { &KING_MIDDLE_GAME_PST };

    score += evaluate_bitboard(game.board.bitboard[Piece::King as usize][Color::White as usize ], king_pst, true);
    score -= evaluate_bitboard(game.board.bitboard[Piece::King as usize][Color::Black as usize], king_pst, false);

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
