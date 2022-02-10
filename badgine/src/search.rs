use cozy_chess::{Board, GameStatus, Move, Piece};
use nohash::IntSet;
use rand::prelude::*;

use crate::eval::evaluate;

pub struct Node {
    eval: Eval,
    depth: usize,
    children: Option<Vec<(Move, Node)>>,
}

impl Node {
    pub fn new(board: &Board, history: &mut IntSet<u64>) -> Self {
        let (eval, children) = match board.status() {
            GameStatus::Won => (Eval::Lost(0), Some(vec![])),
            GameStatus::Drawn => (Eval::Value(0), Some(vec![])),
            GameStatus::Ongoing => {
                if history.contains(&board.hash()) {
                    (Eval::Value(0), Some(vec![]))
                } else if draw_by_insufficient_material(board) {
                    (Eval::Value(0), Some(vec![]))
                } else {
                    (Eval::Value(evaluate(board)), None)
                }
            }
        };
        Node {
            eval,
            depth: 0,
            children,
        }
    }

    pub fn eval(&self) -> Eval {
        self.eval
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn get_pv(&self, pv: &mut Vec<Move>) {
        if let Some((best, next)) = self.children.as_ref().and_then(|c| c.first()) {
            pv.push(*best);
            next.get_pv(pv);
        }
    }

    pub fn search(&mut self, mut board: Board, history: &mut IntSet<u64>) -> usize {
        match self.children.as_mut() {
            Some(children) => {
                if children.is_empty() {
                    return 0;
                }

                let hash = board.hash();
                history.insert(hash);

                let i = (-thread_rng().gen::<f64>().ln() / 0.3) as usize % children.len();
                board.play_unchecked(children[i].0);
                let new_nodes = children[i].1.search(board, history);
                self.depth = self.depth.max(children[i].1.depth + 1);

                history.remove(&hash);
                children.sort_by_key(|(_, n)| n.eval);
                self.eval = -children[0].1.eval.count_time();

                new_nodes
            }
            None => {
                let children = self.children.insert(vec![]);

                history.insert(board.hash());
                board.generate_moves(|mvset| {
                    for mv in mvset {
                        let mut board = board.clone();
                        board.play_unchecked(mv);
                        children.push((mv, Node::new(&board, history)));
                    }
                    false
                });
                history.remove(&board.hash());

                children.shuffle(&mut thread_rng());
                children.sort_by_key(|(_, n)| n.eval);
                self.eval = -children[0].1.eval.count_time();

                children.len()
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Eval {
    Won(u32),
    Value(i32),
    Lost(u32),
}

impl Ord for Eval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Eval::*;
        match (self, other) {
            (Won(d1), Won(d2)) => d2.cmp(d1),
            (Won(_), _) => std::cmp::Ordering::Greater,
            (Value(_), Won(_)) => std::cmp::Ordering::Less,
            (Value(v1), Value(v2)) => v1.cmp(v2),
            (Value(_), Lost(_)) => std::cmp::Ordering::Greater,
            (Lost(d1), Lost(d2)) => d1.cmp(d2),
            (Lost(_), _) => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for Eval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Eval {}

impl Eval {
    fn count_time(self) -> Eval {
        match self {
            Eval::Won(d) => Eval::Won(d + 1),
            Eval::Value(v) => Eval::Value(v),
            Eval::Lost(d) => Eval::Lost(d + 1),
        }
    }
}

impl std::ops::Neg for Eval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Eval::Won(d) => Eval::Lost(d),
            Eval::Value(v) => Eval::Value(-v),
            Eval::Lost(d) => Eval::Won(d),
        }
    }
}

impl std::fmt::Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Eval::Won(d) => write!(f, "mate {}", d / 2 + 1),
            Eval::Value(v) => write!(f, "cp {}", v),
            Eval::Lost(d) => write!(f, "mate -{}", d / 2 + 1),
        }
    }
}

fn draw_by_insufficient_material(board: &Board) -> bool {
    board.pieces(Piece::Pawn).is_empty()
        && board.pieces(Piece::Rook).is_empty()
        && board.pieces(Piece::Queen).is_empty()
        && board.pieces(Piece::Bishop).popcnt() + board.pieces(Piece::Knight).popcnt() < 2
}
