//
// parser - snowflake's parser
//
// copyright (c) 2020 the snowflake authors <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

#[macro_use]
extern crate lalrpop_util;
use logos::Logos;
pub mod ast;
pub mod indentation;
pub mod token;

use token::Token;

lalrpop_mod!(pub snowflake);

// pub fn parse<'a>(
//     input: &'a str,
// ) -> Result<ast::Statement, lalrpop_util::ParseError<usize, token::Token, String>> {
//     let input = token::Token::lexer(input)
//         .spanned()
//         .map(spanned_token_into_item);
//     let mut indentation = indentation::IndentationLevel::new();
//     // todo: make into ProgramParser
//     snowflake::StatementParser::new().parse(&mut indentation, input)
// }

#[cfg(test)]
mod test {
    use super::*;
    use ast;
    use ast::Expression;
    use ast::OpSymbol;
    use ast::Statement;
    use ast::Type;
    use indoc::indoc;
    use num_bigint::BigInt;
    use snowflake::*;

    impl From<isize> for ast::Expression {
        fn from(i: isize) -> Self {
            ast::Expression::Integer(BigInt::from(i))
        }
    }

    impl<'a> From<&'a str> for ast::Expression {
        fn from(s: &'a str) -> Self {
            ast::Expression::Identifier(String::from(s))
        }
    }

    impl From<isize> for ast::Type {
        fn from(i: isize) -> Self {
            ast::Type::Nat(BigInt::from(i))
        }
    }

    impl<'a> From<&'a str> for ast::Type {
        fn from(s: &'a str) -> Self {
            ast::Type::Identifier(String::from(s))
        }
    }

    impl From<isize> for ast::Statement {
        fn from(i: isize) -> Self {
            ast::Statement::Expression(i.into())
        }
    }

    impl<'a> From<&'a str> for ast::Statement {
        fn from(s: &'a str) -> Self {
            ast::Statement::Expression(s.into())
        }
    }

    // test parse for
    macro_rules! test_parse {
        ($path:ty where $($input:expr => $test:expr),*) => {
            $({
                let input = token::lex($input);
                let program = <$path>::new().parse(input).unwrap();
                assert_eq!(program, $test)
            })*
        };
    }

    fn ops(
        l: impl Into<ast::Expression>,
        op: ast::OpSymbol,
        r: impl Into<ast::Expression>,
    ) -> ast::Expression {
        ast::Expression::OpCall {
            op: op,
            args: vec![Box::new(l.into()), Box::new(r.into())],
        }
    }

    #[test]
    fn parse_identifier() {
        test_parse! {
            IdentifierParser where
            "name" =>
                String::from("name"),
            "name_with_underscores_numbers_and_is_long_1234" =>
                String::from("name_with_underscores_numbers_and_is_long_1234")
        }
    }

    #[test]
    fn parse_integer() {
        test_parse! {
          IntegerParser where
          "132" => BigInt::from(132),
          "123_456_789" => BigInt::from(123_456_789)
        }
    }

    #[test]
    fn parse_literal() {
        test_parse! {
          LiteralParser where
          "132" => Expression::Integer(
            BigInt::from(132)
          ),
          "123_456_789" => Expression::Integer(
            BigInt::from(123_456_789)
          ),
          "name" => Expression::Identifier(
            String::from("name")
          ),
          "name_with_underscores_numbers_and_is_long_1234" => Expression::Identifier(
            String::from("name_with_underscores_numbers_and_is_long_1234")
          )
        }
    }

    #[test]
    fn parse_op_call() {
        test_parse! {
            OpCallParser where
            "1 + 1" => ast::Expression::OpCall {
                op: ast::OpSymbol::Plus,
                args: vec![
                    Box::new(1.into()),
                    Box::new(1.into()),
                ]
            },
            // should parse as
            // (1 + (2 * (3 - (4 / 5))))
            // although alternatively it could be made to parse like
            // ((((1 + 2) * 3) - 4) / 5)
            "1 + 2 * 3 - 4 / 5" => {
                use ast::OpSymbol::*;
                ops(1, Plus, ops(2, Star, ops(3, Minus, ops(4, ForwardSlash, 5))))
            }
        }
    }

    #[test]
    fn parse_block() {
        test_parse! {
            BlockParser where
            "\n  123\n  abc\n  123\n " => vec![
                Box::new(123.into()),
                Box::new("abc".into()),
                Box::new(123.into()),
            ]
        }
    }

    #[test]
    fn parse_fn_decl() {
        test_parse! {
            FnDeclParser where
            "add a b => a + b\n" => Statement::FnDecl {
                name: String::from("add"),
                args: vec![
                    String::from("a"),
                    String::from("b")
                ],
                body: vec![
                    Box::new(Statement::Expression(
                        Expression::OpCall {
                            op: ast::OpSymbol::Plus,
                            args: vec![
                                Box::new(Expression::from("a")),
                                Box::new(ast::Expression::from("b"))
                            ]
                        }
                    ))
                ]
            },
            "add a b =>\n  a + b\n" => Statement::FnDecl {
                name: "add".into(),
                args: vec!["a".into(), "b".into()],
                body: vec![
                    Box::new(Statement::Expression(
                        Expression::OpCall {
                            op: ast::OpSymbol::Plus,
                            args: vec![
                                Box::new("a".into()),
                                Box::new("b".into())
                            ]
                        }
                    ))
                ]
            },
            "exp a b => (a * a) + (b * b)\n" => Statement::FnDecl {
                name: "exp".into(),
                args: vec!["a".into(), "b".into()],
                body: vec![
                    Box::new(Statement::Expression(ops(
                        ops("a", OpSymbol::Star, "a"),
                        OpSymbol::Plus,
                        ops("b", OpSymbol::Star, "b")
                    )))
                ]
            },
            "exp a b =>\n  (a * a) + (b * b)\n" => Statement::FnDecl {
                name: "exp".into(),
                args: vec!["a".into(), "b".into()],
                body: vec![
                    Box::new(Statement::Expression(ops(
                        ops("a", OpSymbol::Star, "a"),
                        OpSymbol::Plus,
                        ops("b", OpSymbol::Star, "b")
                    )))
                ]
            }
        }
    }

    #[test]
    fn parse_fn_call() {
        test_parse! {
            FnCallParser where
            "add 1 2" => Expression::FnCall {
                name: "add".into(),
                args: vec![
                    1.into(),
                    2.into(),
                ]
            },
            "add a b" => Expression::FnCall {
                name: "add".into(),
                args: vec![
                    "a".into(),
                    "b".into(),
                ]
            }
        }
    }

    #[test]
    fn parse_expression() {
        test_parse! {
            ExpressionParser where
            "1 + 2" => ops(1, OpSymbol::Plus, 2),
            "1 + (2 * 3)" => ops(1, OpSymbol::Plus, ops(2, OpSymbol::Star, 3)),
            "(1 + 2) * 3" => ops(ops(1, OpSymbol::Plus, 2), OpSymbol::Star, 3)
        }
    }

    #[test]
    fn parse_statement() {
        test_parse! {
            StatementParser where
            "add :: int int -> int" => Statement::TypeDecl {
                name: "add".into(),
                body: Type::FnSig {
                    args: vec![
                        Box::new("int".into()),
                        Box::new("int".into())
                    ],
                    ret: Box::new("int".into())
                },
            },
            "add a b => a + b\n" => Statement::FnDecl {
                name: String::from("add"),
                args: vec![
                    String::from("a"),
                    String::from("b")
                ],
                body: vec![
                    Box::new(Statement::Expression(Expression::OpCall {
                        op: ast::OpSymbol::Plus,
                        args: vec![
                            Box::new(Expression::from("a")),
                            Box::new(ast::Expression::from("b"))
                        ]
                    }))
                ]
            }
        }
    }

    // #[test]
    // fn parse_program() {
    //     let type_decl_input = indoc! {"
    //         fib :: isize -> isize
    //     "};

    //     let fn_decl_input = indoc! {"
    //         fib n =>
    //             (fib n - 1) + (fib n - 2)
    //     "};

    //     test_parse! {
    //         ProgramParser where
    //         "" => vec![],
    //         type_decl_input => vec![
    //             Statement::TypeDecl {
    //                 name: "fib".into(),
    //                 body: Type::FnSig {
    //                     args: vec![
    //                         Box::new("isize".into())
    //                     ],
    //                     ret: Box::new("isize".into())
    //                 }
    //             }
    //         ],
    //         fn_decl_input => vec![
    //             Statement::FnDecl {
    //                 name: "fib".into(),
    //                 args: vec!["n".into()],
    //                 body: vec![
    //                     Box::new(ops(
    //                         Expression::FnCall {
    //                             name: "fib".into(),
    //                             args: vec![
    //                                 ops("n", OpSymbol::Minus, 1)
    //                             ]
    //                         },
    //                         OpSymbol::Plus,
    //                         Expression::FnCall {
    //                             name: "fib".into(),
    //                             args: vec![
    //                                 ops("n", OpSymbol::Minus, 2)
    //                             ]
    //                         },
    //                     ))
    //                 ]
    //             }
    //         ]
    //     }
    // }

    // TypeExpression tests.

    #[test]
    fn parse_fn_sig() {
        test_parse! {
            FnSigParser where
            "int int -> int" => Type::FnSig {
                args: vec![
                    Box::new("int".into()),
                    Box::new("int".into())
                ],
                ret: Box::new("int".into())
            },
            "int int -> int -> int" => Type::FnSig {
                args: vec![
                    Box::new("int".into()),
                    Box::new("int".into())
                ],
                ret: Box::new(Type::FnSig {
                    args: vec![
                        Box::new("int".into())
                    ],
                    ret: Box::new("int".into())
                })
            }
        }
    }

    #[test]
    fn parse_type_decl() {
        test_parse! {
            TypeDeclParser where
            "fib :: int int -> int" => Statement::TypeDecl {
                name: "fib".into(),
                body: Type::FnSig {
                    args: vec![
                        Box::new("int".into()),
                        Box::new("int".into())
                    ],
                    ret: Box::new("int".into())
                },
            }
        }
    }
}
