use clap::{Parser, Subcommand};
use fql::{ast::Expr, parse};

#[derive(Subcommand)]
enum Cmd {
    Facts,
    ListProperties,
    ListOperands,
}

#[derive(Parser)]
struct Opts {
    #[clap(name = "FILTER")]
    filter: String,
    #[clap(subcommand)]
    command: Cmd,
}

impl Opts {
    fn run(&self) {
        let expr = parse(&self.filter).to_expr().unwrap();
        match self.command {
            Cmd::Facts => match expr {
                Expr::Binary(_) => println!("binary"),
                Expr::Paren(_) => println!("parenthesized"),
                Expr::Clause(_) => println!("clause"),
            },
            Cmd::ListProperties => {
                for clause in expr.clauses() {
                    if let Some(property) = clause.property() {
                        println!("{}", property);
                    }
                }
            }
            Cmd::ListOperands => {
                for clause in expr.clauses() {
                    if let Some(operand) = clause.operand() {
                        println!("{}", operand);
                    }
                }
            }
        }
    }
}

fn main() {
    Opts::parse().run();
}
