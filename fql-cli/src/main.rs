use std::collections::BTreeSet;

use clap::{Parser, Subcommand};
use fql::{
    ast::{Expr, Lit},
    parse,
};

#[derive(Subcommand)]
enum Cmd {
    Facts,
    /// List the properties in FILTER (e.g. "host.online").
    ListProperties,
    /// List the operand values in FILTER (e.g. "true" or "'windows'").
    ListOperands,
    /// Print the debug representation of the tree, and any diagnostics.
    PrintTree,
    /// List the unique literal values in FILTER in sorted order by type then value.
    SortLiterals,
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
        let parse_result = parse(&self.filter);
        let expr = parse_result.to_expr();
        match self.command {
            Cmd::Facts => match expr.unwrap() {
                Expr::Binary(_) => println!("binary"),
                Expr::Paren(_) => println!("parenthesized"),
                Expr::Clause(_) => println!("clause"),
            },
            Cmd::ListProperties => {
                for clause in expr.unwrap().clauses() {
                    if let Some(property) = clause.property() {
                        println!("{}", property);
                    }
                }
            }
            Cmd::ListOperands => {
                for clause in expr.unwrap().clauses() {
                    if let Some(operand) = clause.operand() {
                        println!("{}", operand);
                    }
                }
            }
            Cmd::PrintTree => {
                println!("{}", parse_result.debug_tree());

                for diagnostic in parse_result.diagnostics() {
                    println!("{diagnostic}");
                }
            }
            Cmd::SortLiterals => {
                let mut bools = BTreeSet::<bool>::new();
                let mut ints = BTreeSet::<u64>::new();
                let mut strings = BTreeSet::<String>::new();

                for value in expr
                    .unwrap()
                    .clauses()
                    .filter_map(|c| c.operand()?.literal()?.value())
                {
                    match value {
                        Lit::Str(s) => strings.insert(s.value().into_owned()),
                        Lit::Bool(b) => bools.insert(b.value()),
                        Lit::Int(i) => match i.value() {
                            Ok(i) => ints.insert(i),
                            Err(_) => false,
                        },
                    };
                }

                for b in bools {
                    println!("{b}");
                }

                for i in ints {
                    println!("{i}");
                }

                for s in strings {
                    println!("{s}");
                }
            }
        }
    }
}

fn main() {
    Opts::parse().run();
}
