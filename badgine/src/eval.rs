use std::collections::VecDeque;

use cozy_chess::{get_king_moves, BitBoard, Board, Color, Piece};

pub fn evaluate(board: &Board) -> i32 {
    material(board) + king_space(board, board.side_to_move())
        - king_space(board, !board.side_to_move())
        + pawn_advancedness(board, board.side_to_move())
        - pawn_advancedness(board, !board.side_to_move())
}

fn material(board: &Board) -> i32 {
    let pawns = board.pieces(Piece::Pawn);
    let minors = board.pieces(Piece::Knight) | board.pieces(Piece::Bishop);
    let rooks = board.pieces(Piece::Rook);
    let queens = board.pieces(Piece::Queen);
    let ours = board.colors(board.side_to_move());
    let theirs = !ours;
    let pawns = (ours & pawns).popcnt() as i32 - (pawns & theirs).popcnt() as i32;
    let minors = (ours & minors).popcnt() as i32 - (minors & theirs).popcnt() as i32;
    let rooks = (ours & rooks).popcnt() as i32 - (rooks & theirs).popcnt() as i32;
    let queens = (ours & queens).popcnt() as i32 - (queens & theirs).popcnt() as i32;
    100 * pawns + 300 * minors + 500 * rooks + 900 * queens
}

fn king_space(board: &Board, color: Color) -> i32 {
    let opponent_attacks = get_attack_set(board, !color);

    let mut reachable = BitBoard::EMPTY;
    let mut queue = VecDeque::new();
    queue.push_back(board.king(color));
    while let Some(sq) = queue.pop_front() {
        let hits = get_king_moves(sq) & !opponent_attacks & !reachable;
        reachable |= hits;
        for sq in hits {
            queue.push_back(sq);
        }
    }

    reachable.popcnt() as i32
}

fn get_attack_set(board: &Board, color: Color) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;
    for sq in board.pieces(Piece::Pawn) & board.colors(color) {
        attacks |= cozy_chess::get_pawn_attacks(sq, color);
    }
    for sq in (board.pieces(Piece::Rook) | board.pieces(Piece::Queen)) & board.colors(color) {
        attacks |= cozy_chess::get_rook_moves(sq, board.occupied());
    }
    for sq in (board.pieces(Piece::Bishop) | board.pieces(Piece::Queen)) & board.colors(color) {
        attacks |= cozy_chess::get_bishop_moves(sq, board.occupied());
    }
    for sq in board.pieces(Piece::Knight) & board.colors(color) {
        attacks |= cozy_chess::get_knight_moves(sq);
    }
    for sq in board.pieces(Piece::King) & board.colors(color) {
        attacks |= cozy_chess::get_king_moves(sq);
    }
    attacks
}

fn pawn_advancedness(board: &Board, color: Color) -> i32 {
    const RANK_SCORES: [i32; 8] = [0, 0, 4, 10, 15, 19, 25, 0];
    let mut score = 0;
    for sq in board.pieces(Piece::Pawn) | board.colors(color) {
        score += RANK_SCORES[sq.rank().relative_to(color) as usize];
    }
    score
}
