use nom::*;
use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

use crate::ast::*;

type Span<'a> = LocatedSpan<CompleteStr<'a>>;

named!(atom(Span) -> Atom, map!(
    preceded!(char!('\''), alpha1),
    |LocatedSpan {fragment: CompleteStr(s), ..}| Atom(s)
));
