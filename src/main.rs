use std::env::args;

use parser::prelude::*;

fn label() -> impl Fn(StringReader) -> ParserOut<String> {
    take_fold(String::new(), |s, c, i| {
        if c.is_alphabetic() {
            ControlFlow::Continue(format!("{s}{c}"))
        } else {
            ControlFlow::Break(if s.is_empty() {
                Err(ParserError::NoMatch { head: i.true_index(0) })
            } else {
                Ok((i, s))
            })
        }
    })
}

fn dynamic_var_func_parser_v2() -> impl Fn(StringReader) -> ParserOut<(String, usize)> {
    delimited(
        "func(",
        label(),
        ")"
    ).and_then(|input, label| {
        preceded((
            white,
            "=",
            white
        ), separated_pair(label.case_insensitive(), (white, "^", white), unsigned.map_ok(|i| i as usize)))(input)
    })
}

fn test<S: Into<StringReader>>(expr: S) {
    let (var, exp) = expr.parse_with(false, dynamic_var_func_parser_v2()).unwrap();
    println!("{var} ^ {exp}");
}

fn main() {
    if args().len() != 2 {
        println!("expected a single argument");
        return;
    }
    test(args().last().unwrap());
}