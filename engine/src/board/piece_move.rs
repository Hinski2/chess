
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct PieceMove {
    pub from: u8, 
    pub to: u8,
    pub flag: MoveFlag,
}
