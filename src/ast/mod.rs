mod list;

pub use list::List;

pub type ASTList = list::List<AtomOrList>;

#[derive(Debug)]
pub enum AtomOrList {
    Atom(Atom, usize),
    List(List<AtomOrList>, usize),
}

#[derive(Debug)]
pub enum Atom {
    AIdentifier(String),
    AString(String),
    // AFloat(f64),
    AInteger(isize),
    ATrue,
    AFalse,
    // AVector(),
    // AChar(char),
}




