use std::fmt;
#[derive(PartialEq, Eq, Copy ,Clone)]
pub enum PlayerSymbol {
    N, // No player
    X, // Player with the symbol X
    O, // Player with the symbol O
}
impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x: &str = match self {
            &PlayerSymbol::N => " ",
            &PlayerSymbol::X => "X",
            &PlayerSymbol::O => "O",
        };
        write!(f, "{}", x)
    }
}
