use nom::*;
use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

use crate::ast::*;

pub type Span<'a> = LocatedSpan<CompleteStr<'a>>;

named!(atom(Span) -> Atom, map!(
    recognize!(tuple!(char!('\''), take_while1!(|ch: char| ch.is_ascii_alphabetic() || ch == '-'))),
    |LocatedSpan {fragment: CompleteStr(s), ..}| Atom(s)
));

named!(ident(Span) -> Ident, map!(
    recognize!(tuple!(
        take_while1!(|ch: char| ch.is_ascii_alphabetic() || "+-!$%&*/:<=>?~_^".contains(ch)),
        take_while!(|ch: char| ch.is_ascii_alphanumeric() || ".+-".contains(ch))
    )),
    |id| Ident(id.fragment.0)
));

named!(expr(Span) -> Expr, alt!(
    atom => { Expr::Atom } |
    ident => { Expr::Ident } |
    list => { Expr::List }
));

named!(list(Span) -> Vec<Expr>, ws!(delimited!(
    char!('('),
    many0!(expr),
    char!(')')
)));

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parser {
        ($parser:ident ( $input:expr ) -> ok) => {
            let input = $input.trim();
            let input = Span::new(CompleteStr(input));
            match $parser(input) {
                Ok((remaining, _)) => {
                    assert!(remaining.fragment.0.is_empty(),
                        "fail: parser did not completely read input for: `{}`\nRemaining: `{}`", $input, remaining.fragment.0);
                },
                Err(err) => panic!("parse of `{}` failed. Error: {:?}", $input, err),
            }
        };
        ($parser:ident ( $input:expr ) -> err) => {
            let input = $input.trim();
            let input = Span::new(CompleteStr(input));
            match $parser(input) {
                Ok((ref remaining, ref output)) if remaining.fragment.0.is_empty() => {
                    panic!("parse of `{}` succeeded (when it should have failed). Result: {:?}", $input, output);
                },
                _ => {}, // Expected
            }
        };
        ($parser:ident ( $input:expr ) -> ok, $expected:expr) => {
            let input = $input.trim();
            let input = Span::new(CompleteStr(input));
            match $parser(input) {
                Ok((remaining, output)) => {
                    assert!(remaining.fragment.0.is_empty(),
                        "fail: parser did not completely read input for: `{}`\nRemaining: `{}`", $input, remaining.fragment.0);
                    assert_eq!(output, $expected, "Incorrect result for parse of input: `{}`", $input);
                },
                Err(err) => panic!("parse of `{}` failed. Error: {:?}", $input, err),
            }
        };
    }

    #[test]
    fn test_atom() {
        test_parser!(atom("") -> err);
        test_parser!(atom("'") -> err);
        test_parser!(atom("''") -> err);
        test_parser!(atom("''a") -> err);
        test_parser!(atom("'1") -> err);
        test_parser!(atom("'ad1") -> err);
        test_parser!(atom("'a-d1") -> err);

        test_parser!(atom("'a") -> ok, Atom("'a"));
        test_parser!(atom("'a") -> ok, Atom("'a"));
        test_parser!(atom("'abcdef") -> ok, Atom("'abcdef"));
        test_parser!(atom("'a-very-happy-atom") -> ok, Atom("'a-very-happy-atom"));
        test_parser!(atom("'-") -> ok, Atom("'-"));
        test_parser!(atom("'-a-a-") -> ok, Atom("'-a-a-"));
        test_parser!(atom("'-------") -> ok, Atom("'-------"));
    }

    #[test]
    fn test_expr_unmatched() {
        test_parser!(expr("
            (car
                (cons 'ratatouille 'baguette)
        ") -> err);
        test_parser!(expr("
            (car
                cons 'ratatouille 'baguette))
        ") -> err);
    }

    #[test]
    fn test_expr() {
        test_parser!(expr("
            (car
                (cons 'ratatouille 'baguette))
        ") -> ok, Expr::List(vec![
            Expr::Ident(Ident("car")),
            Expr::List(vec![
                Expr::Ident(Ident("cons")),
                Expr::Atom(Atom("'ratatouille")),
                Expr::Atom(Atom("'baguette")),
            ])
        ]));

        test_parser!(expr("
            (car
                (cdr
                    (cons 'ratatouille
                        (cons 'baguette 'olive-oil))))
        ") -> ok);

        test_parser!(expr("
            (Pair Atom Atom)
        ") -> ok);

        test_parser!(expr("
            (Pair
                (cdr
                    (cons Atom 'olive))
                (car
                    (cons 'oil Atom)))
        ") -> ok);

        test_parser!(expr("
            (+ (add1
                (add1 zero))
              (add1 zero))
        ") -> ok);

        test_parser!(expr("
            (define one
                (add1 zero))
        ") -> ok);

        test_parser!(expr("
            (claim two
                Nat)
        ") -> ok);
    }
}
