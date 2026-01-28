use engine::board::piece_move::PieceMove;

pub trait INode {
    fn clear(&mut self);    
    fn len(&self) -> usize;     // zwraca liczbę ruchów
    fn is_empty(&self) -> bool; // czy mozemy zrobić jakiś ruch
    fn set_moves(&mut self, moves: Vec<PieceMove>);
    fn take_moves(&mut self) -> Vec<PieceMove>;

    fn as_slice(&self) -> &[PieceMove];
    fn as_slice_mut(&mut self) -> &mut [PieceMove];
}
