use std::borrow::Cow;
use std::process::exit;

use parser::ast::{Expression, Type, OpSymbol, Statement};
use tag::{UniverseBuilder, UniverseError, Universe, BindingBuilder, Binding};

fn eval_statement<'a>(universe: &mut Universe<'a, Statement>, statement: Statement) -> Result<(), Box<dyn std::error::Error>> {
    match statement {
        Statement::FnDecl { name: n, args: a, body: b } => {
            let r = universe.insert(|x| x
                                            .set_name(Cow::from(n.clone()))
                                            .set_value(Statement::FnDecl {
                                                name: n.clone(),
                                                args: a.clone(),
                                                body: b.clone(),
                                            })
                                            );
            //let _ = match e {
            //    Ok(o) => o,
            //    // TODO(@monarrk): handle this error better probably?
            //    Err(err) if matches!(err, UniverseError::BindingAlreadyExists) => {
            //        eprintln!("Error: Binding already exists: {}", err);
            //        exit(1);
            //    },
            //    Err(err) => {
            //        eprintln!("Error!: {}", err);
            //        return ;
            //    },
            //};
            
            return match r {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }

        },

        Statement::TypeDecl { name: n, body: b } => {
            match b {
                Type::FnSig { args: a, ret: r } => {
                    
                },
                Type::Nat(b) => {

                },
                Type::Identifier(s) => {

                },
            };
        },

        Statement::Expression(e) => {
            match e {
                Expression::OpCall { op: o, args: a } => {
                    match o {
                        OpSymbol::Plus => {

                        },
                        OpSymbol::Minus => {

                        },
                        OpSymbol::Star => {

                        },
                        OpSymbol::ForwardSlash => {

                        },
                    };
                },

                Expression::FnCall { name: n, args: a } => {
                    
                },

                Expression::Integer(i) => {

                },

                Expression::Identifier(i) => {

                },
            };
        },
    };

    Ok(())
}

/// Execute a snowflake AST
pub fn eval(statement: Statement) -> Result<(), Box<dyn std::error::Error>> {
    let mut universe = Universe::<Statement>::default();
    eval_statement(&mut universe, statement)
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::snowflake::*;
    use parser::lexer;

    #[test]
    fn eval_fn_decl() {
        eval(FnDeclParser::new().parse(lexer::lex("add a b =>\n  a + b\n")).unwrap()).unwrap();
    }
}
