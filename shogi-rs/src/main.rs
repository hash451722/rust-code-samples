use shogi::{Move, Position};
use shogi::bitboard::Factory as BBFactory;
use shogi::square::consts::*;
fn main() {
    
    BBFactory::init();
    let mut pos = Position::new();
    
    // Position can be set from the SFEN formatted string.
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
    
    // You can programatically create a Move instance.
    let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
    pos.make_move(m).unwrap();
    
    // Move can be created from the SFEN formatted string as well.
    let m = Move::from_sfen("7c7d").unwrap();
    pos.make_move(m).unwrap();
    
    // Position can be converted back to the SFEN formatted string.
    assert_eq!("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f 7c7d", pos.to_sfen());

    println!("{}", pos.to_sfen())
}
