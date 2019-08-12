mod list;

pub use list::List;

pub type ASTList = list::List<ASTAtom>;
pub type ASTAtom = (Atom, usize);

#[derive(Debug, Clone)]
pub enum Atom {
    AIdentifier(String),
    AString(String),
    // AFloat(f64),
    AInteger(isize),
    ATrue,
    AFalse,
    AList(list::List<Atom>),
    // AVector(),
    // AMap(),
    // AChar(char),
}




