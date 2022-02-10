use cozy_chess::{Board, File, Move, Piece, Square};
use nohash::{IntMap, IntSet};

use crate::search::Node;

mod search;
mod eval;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    if buf.trim() != "uci" {
        eprintln!("Expected first command to be 'uci'");
        std::process::exit(1);
    }
    println!("id name badgine 0.1.0");
    println!("id author MinusKelvin");
    println!("uciok");

    let mut board = Board::default();
    let mut history = IntSet::default();

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        if buf.trim().is_empty() {
            continue;
        }
        let mut params = buf.split_ascii_whitespace();
        match params.next().unwrap() {
            "isready" => {
                println!("readyok");
            }
            "position" => {
                let position = params.next().unwrap();
                board = match position {
                    "startpos" => cozy_chess::Board::default(),
                    _ => panic!("fen position unsupported"),
                };
                let mut history_counts = IntMap::<_, i32>::default();
                if params.next() == Some("moves") {
                    for mv in params.map(|m| m.parse::<Move>().unwrap()) {
                        *history_counts.entry(board.hash()).or_default() += 1;
                        let mv = from_uci_castling(&board, mv);
                        board.play(mv);
                    }
                }
                history = history_counts
                    .into_iter()
                    .filter(|&(_, c)| c > 1)
                    .map(|(h, _)| h)
                    .collect();
            }
            "go" => {
                let mut root = Node::new(&board, &mut history);

                let mut nodes = 0;
                for _ in 0..100 {
                    nodes += root.search(board.clone(), &mut history);
                }

                let mut pv = vec![];
                root.get_pv(&mut pv);
                print!("info score {} depth {} nodes {nodes} pv", root.eval(), root.depth());
                let mut b = board.clone();
                for &mv in &pv {
                    print!(" {}", to_uci_castling(&b, mv));
                    b.play_unchecked(mv);
                }
                println!();
                
                println!("bestmove {}", pv[0]);
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn to_uci_castling(board: &Board, mut mv: Move) -> Move {
    if board.color_on(mv.from) == board.color_on(mv.to) {
        if mv.to.file() > mv.from.file() {
            mv.to = Square::new(File::G, mv.to.rank());
        } else {
            mv.to = Square::new(File::C, mv.to.rank());
        }
    }
    mv
}

fn from_uci_castling(board: &Board, mut mv: Move) -> Move {
    if mv.from.file() == File::E && board.piece_on(mv.from) == Some(Piece::King) {
        if mv.to.file() == File::G {
            mv.to = Square::new(File::H, mv.to.rank());
        } else if mv.to.file() == File::C {
            mv.to = Square::new(File::A, mv.to.rank());
        }
    }
    mv
}
