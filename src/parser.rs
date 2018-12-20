use nom::*;
use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

use crate::ast::*;

type Span<'a> = LocatedSpan<CompleteStr<'a>>;

named!(atom(Span) -> Atom, map!(
    preceded!(char!('\''), take_while1!(|c: char| c.is_alpha() || c == '-')),
    |LocatedSpan {fragment: CompleteStr(s), ..}| Atom(s)
));

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parser {
        ($parser:ident ( $input:expr ) -> ok) => {
            let input = Span::new(CompleteStr($input));
            match $parser(input) {
                Ok((remaining, _)) => {
                    assert!(remaining.fragment.0.is_empty(),
                        "fail: parser did not completely read input for: `{}`\nRemaining: `{}`", $input, remaining.fragment.0);
                },
                Err(err) => panic!("parse of `{}` failed. Error: {:?}", $input, err),
            }
        };
        ($parser:ident ( $input:expr ) -> err) => {
            let input = Span::new(CompleteStr($input));
            match $parser(input) {
                Ok((ref remaining, ref output)) if remaining.fragment.0.is_empty() => {
                    panic!("parse of `{}` succeeded (when it should have failed). Result: {:?}", $input, output);
                },
                _ => {}, // Expected
            }
        };
        ($parser:ident ( $input:expr ) -> ok, $expected:expr) => {
            let input = Span::new(CompleteStr($input));
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
        test_parser!(atom("'") -> err);
        test_parser!(atom("''") -> err);
        test_parser!(atom("''a") -> err);
        test_parser!(atom("'1") -> err);
        test_parser!(atom("'ad1") -> err);
        test_parser!(atom("'a-d1") -> err);

        test_parser!(atom("'a") -> ok, Atom("a"));
        test_parser!(atom("'abcdef") -> ok, Atom("abcdef"));
        test_parser!(atom("'a-very-happy-atom") -> ok, Atom("a-very-happy-atom"));
        test_parser!(atom("'-") -> ok, Atom("-"));
        test_parser!(atom("'-a-a-") -> ok, Atom("-a-a-"));
        test_parser!(atom("'-------") -> ok, Atom("-------"));
    }
}
