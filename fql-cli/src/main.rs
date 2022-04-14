use clap::{Parser, Subcommand};
use fql::{ast::Expr, parse};

#[derive(Subcommand)]
enum Cmd {
    Facts,
    /// List the properties in FILTER (e.g. "host.online").
    ListProperties,
    /// List the operand values in FILTER (e.g. "true" or "'windows'").
    ListOperands,
}

#[derive(Parser)]
struct Opts {
    /// An FQL string, such as "host.online:true".
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
