#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom<'a>(pub &'a str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<'a>(pub &'a str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Atom(Atom<'a>),
    Ident(Ident<'a>),
    List(Vec<Expr<'a>>),
}
