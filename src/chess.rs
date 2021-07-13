use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Colour {
    Black,
    White
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
}

pub type Space = Option<Piece>;
pub type Board = [[Space; 8]; 8];

pub fn empty_board() -> Board {
    [[None; 8]; 8]
}

pub fn initial_board() -> Board {
    [
        [
            Some(Piece{piece_type: PieceType::Rook, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::Knight, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::Bishop, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::King, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::Queen, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::Bishop, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::Knight, colour: Colour::Black}),
            Some(Piece{piece_type: PieceType::Rook, colour: Colour::Black}),
        ], [
            Some(Piece{piece_type: PieceType::Pawn, colour: Colour::Black}); 8
        ], [
            None ; 8
        ], [
            None ; 8
        ], [
            None ; 8
        ], [
            None ; 8
        ],[
            Some(Piece{piece_type: PieceType::Pawn, colour: Colour::White}); 8
        ], [
                Some(Piece{piece_type: PieceType::Rook, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::Knight, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::Bishop, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::King, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::Queen, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::Bishop, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::Knight, colour: Colour::White}),
                Some(Piece{piece_type: PieceType::Rook, colour: Colour::White}),
            ]
    ]
}
