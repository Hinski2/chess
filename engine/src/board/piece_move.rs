
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveFlag {
    PromoteToQueenAndCapture,
    PromoteToRookAndCapture,
    PromoteToBishopAndCapture,
    PromoteToKnightAndCapture,

    PromoteToQueen,
    PromoteToRook,
    PromoteToBishop,
    PromoteToKnight,

    Capture,
    EnPassantCapture,

    Castling,
    DoublePawnPush,
    Normal, 

    None, 
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceMove {
    pub from: u8, 
    pub to: u8,
    pub flag: MoveFlag,
}