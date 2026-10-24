use engine::game::game::Game;
use engine::board::{Piece, Color};

use super::{PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE};
use super::{PAWN_PST, KNIGHT_PST, BISHOP_PST, ROOK_PST, QUEEN_PST, KING_END_GAME_PST, KING_MIDDLE_GAME_PST};

pub(super) fn evaluate_bitboard(mut bitboard: u64, pst: &[i32; 64], is_white: bool) -> i32 {
    let mut score: i32 = 0;
    
    while bitboard != 0 {
        let sq = bitboard.trailing_zeros() as usize;
        bitboard &= bitboard - 1;

        if is_white {
            score += pst[sq ^ 56];
        } else {
            score += pst[sq];
        }
    }

    score
}

pub(super) fn evaluate_pos(game: &Game) -> i32 {
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

pub(super) fn evaluate_pawns(game: &Game) -> i32 {
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