use std::{ops::Range, path::PathBuf};

use ariadne::{Label, Report, ReportKind};
use rowan::NodeOrToken;
use rustre_parser::{lexer::Token, SyntaxNode, SyntaxToken, ast::AstNode};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Dumps AST generated from file
    Ast { file: Option<String> },
    // Compiles file
    Build { file: Option<String> },
}

fn main() -> Result<(), u8> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ast { file } => {
            match file {
                Some(filename) => {
                    let path = PathBuf::from(filename);

                    // TODO: there is a bug in logos, the last token is not always
                    // properly parsed when there is a missing line break at the end
                    // of the file

                    let mut driver = rustre_core::driver();
                    rustre_core::add_source_file(&mut driver, path);
                    for file in &*rustre_core::files::files::query(&driver, ()) {
                        dbg!(&file.path);
                        let ast = rustre_core::parse_file(&driver, file);
                        print(0, NodeOrToken::Node(ast.syntax().clone()));
                    }

                    // TODO: re-introduce error handling when they are reported with salsa
                    /*let no_errors = errors.is_empty();
                
                    for err in errors {
                        Report::<(String, Range<usize>)>::build(ReportKind::Error, filename.clone(), err.span.start())
                            .with_message(err.msg)
                            .with_label(Label::new((filename.clone(), err.span)))
                            .finish()
                            .print(ariadne::sources(vec![(filename.clone(), &contents)]))
                            .unwrap();
                    }
                
                    if no_errors {
                        Ok(())
                    } else {
                        Err(1)
                    }*/
                    Ok(())
                }
                None => {
                    println!("Missing argument : file");
                    Err(1)
                }     
            }
        }
        Commands::Build { file } => {
            match file {
                Some(_filename) => {
                    println!("Ça construit un compilateur ou quoi ?? (PAS ENCORE PRÊT)");
                    Ok(())
                }
                None => {
                    println!("Missing argument : file");
                    Err(1)
                }
            }
        }
    }
}

fn print(indent: usize, element: rowan::NodeOrToken<SyntaxNode, SyntaxToken>) {
    let kind: Token = element.kind().into();
    print!("{:indent$}", "", indent = indent);
    match element {
        rowan::NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        }

        NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
    }
}
