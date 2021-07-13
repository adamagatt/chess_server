use std::collections::HashMap;
use crate::chess::{Board, Colour, Piece, PieceType, Space};

type SpaceMap = HashMap<Space, char>;

const TOP_LINE: &str =    "┏━┳━┳━┳━┳━┳━┳━┳━┓\n";
const MIDDLE_LINE: &str = "\n┣━╋━╋━╋━╋━╋━╋━╋━┫\n";
const BOTTOM_LINE: &str = "\n┗━┻━┻━┻━┻━┻━┻━┻━┛";
const SEP: char = '┃';
const BLANK: char = ' ';

pub fn get_space_map() -> SpaceMap {
    [
        (Some(Piece{piece_type: PieceType::Pawn, colour: Colour::Black}), 'p'),
        (Some(Piece{piece_type: PieceType::Rook, colour: Colour::Black}), 'r'),
        (Some(Piece{piece_type: PieceType::Bishop, colour: Colour::Black}), 'b'),
        (Some(Piece{piece_type: PieceType::Knight, colour: Colour::Black}), 'n'),
        (Some(Piece{piece_type: PieceType::Queen, colour: Colour::Black}), 'q'),
        (Some(Piece{piece_type: PieceType::King, colour: Colour::Black}), 'k'),
        (Some(Piece{piece_type: PieceType::Pawn, colour: Colour::White}), 'P'),
        (Some(Piece{piece_type: PieceType::Rook, colour: Colour::White}), 'R'),
        (Some(Piece{piece_type: PieceType::Bishop, colour: Colour::White}), 'B'),
        (Some(Piece{piece_type: PieceType::Knight, colour: Colour::White}), 'N'),
        (Some(Piece{piece_type: PieceType::Queen, colour: Colour::White}), 'Q'),
        (Some(Piece{piece_type: PieceType::King, colour: Colour::White}), 'K'),
        (None, ' ')
    ].iter().cloned().collect()
}

pub fn to_text(board: Board, space_map: SpaceMap) -> String {
    String::from(TOP_LINE) +
    &board.iter().map(
        |row| String::from(SEP)
            + &row
                .iter()
                .map(|&space| space_map.get(&space).unwrap_or(&BLANK))
                .fold(String::with_capacity(16), |mut acc, cur| {acc.push(*cur); acc.push(SEP); acc})
    ).collect::<Vec<String>>().join(MIDDLE_LINE) +
    &String::from(BOTTOM_LINE)
}
//♘