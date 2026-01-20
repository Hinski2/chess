
#[derive(Clone)]
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
}

#[derive(Clone)]
pub struct PieceMove {
    pub from: u8, 
    pub to: u8,
    pub flag: MoveFlag,
}