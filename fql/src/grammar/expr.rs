use crate::{
    lexer::TokenKind,
    parser::{CompletedMarker, Parser},
    syntax::SyntaxKind,
};

use super::{operand, operator, property};

#[derive(Clone, Copy)]
enum CompoundOp {
    And,
    Or,
}

impl CompoundOp {
    fn parse(p: &mut Parser) -> Option<Self> {
        if p.at(TokenKind::Plus) {
            Some(Self::And)
        } else if p.at(TokenKind::Comma) {
            Some(Self::Or)
        } else {
            None
        }
    }

    fn binding_power(self) -> (u8, u8) {
        match self {
            Self::Or => (1, 2),
            Self::And => (3, 4),
        }
    }
}

pub(crate) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

fn expr_binding_power(p: &mut Parser, min_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let (left_power, right_power) = if let Some(op) = CompoundOp::parse(p) {
            op.binding_power()
        } else {
            break;
        };

        if left_power < min_power {
            break;
        }

        p.bump();

        let rhs = expr_binding_power(p, right_power);
        lhs = lhs.precede(p).complete(p, SyntaxKind::InfixExpr);

        if rhs.is_none() {
            break;
        }
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    paren_expr(p).or_else(|| clause(p))
}

fn paren_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LParen) {
        let m = p.start();
        p.bump();
        p.expect_one(expr);
        p.expect(TokenKind::RParen);
        Some(m.complete(p, SyntaxKind::ParenExpr))
    } else {
        None
    }
}

fn clause(p: &mut Parser) -> Option<CompletedMarker> {
    let m = property(p)?.precede(p);

    p.expect(TokenKind::Colon);
    operator(p);
    p.expect_one(operand);

    Some(m.complete(p, SyntaxKind::Clause))
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        crate::parser::check_with(super::expr, input, expected_tree)
    }

    #[test]
    fn empty_parens() {
        check(
            "()",
            expect![[r#"
                Root@0..2
                  ParenExpr@0..2
                    LParen@0..1 "("
                    RParen@1..2 ")"

                At 1..2, expected '(', or ident, found ')'"#]],
        )
    }

    #[test]
    fn incomplete_compound() {
        check(
            "host.online:true+",
            expect![[r#"
                Root@0..17
                  InfixExpr@0..17
                    Clause@0..16
                      Property@0..11
                        Ident@0..4 "host"
                        Period@4..5 "."
                        Ident@5..11 "online"
                      Colon@11..12 ":"
                      Operand@12..16
                        Literal@12..16
                          Boolean@12..16 "true"
                    Plus@16..17 "+""#]],
        );
    }

    #[test]
    fn chained_and() {
        check(
            "host:'windows'+online:'today'+sensor_version:'current'",
            expect![[r#"
                Root@0..54
                  InfixExpr@0..54
                    InfixExpr@0..29
                      Clause@0..14
                        Property@0..4
                          Ident@0..4 "host"
                        Colon@4..5 ":"
                        Operand@5..14
                          Literal@5..14
                            String@5..14 "'windows'"
                      Plus@14..15 "+"
                      Clause@15..29
                        Property@15..21
                          Ident@15..21 "online"
                        Colon@21..22 ":"
                        Operand@22..29
                          Literal@22..29
                            String@22..29 "'today'"
                    Plus@29..30 "+"
                    Clause@30..54
                      Property@30..44
                        Ident@30..44 "sensor_version"
                      Colon@44..45 ":"
                      Operand@45..54
                        Literal@45..54
                          String@45..54 "'current'""#]],
        )
    }

    #[test]
    fn interspersed_and_or() {
        check(
            "host:'windows',online:'today'+sensor_version:'current'",
            expect![[r#"
                Root@0..54
                  InfixExpr@0..54
                    Clause@0..14
                      Property@0..4
                        Ident@0..4 "host"
                      Colon@4..5 ":"
                      Operand@5..14
                        Literal@5..14
                          String@5..14 "'windows'"
                    Comma@14..15 ","
                    InfixExpr@15..54
                      Clause@15..29
                        Property@15..21
                          Ident@15..21 "online"
                        Colon@21..22 ":"
                        Operand@22..29
                          Literal@22..29
                            String@22..29 "'today'"
                      Plus@29..30 "+"
                      Clause@30..54
                        Property@30..44
                          Ident@30..44 "sensor_version"
                        Colon@44..45 ":"
                        Operand@45..54
                          Literal@45..54
                            String@45..54 "'current'""#]],
        )
    }

    #[test]
    fn complete_compound() {
        check(
            "(host.online:true,host.last_online:'today')+host.sensor_version:'current'",
            expect![[r#"
                Root@0..73
                  InfixExpr@0..73
                    ParenExpr@0..43
                      LParen@0..1 "("
                      InfixExpr@1..42
                        Clause@1..17
                          Property@1..12
                            Ident@1..5 "host"
                            Period@5..6 "."
                            Ident@6..12 "online"
                          Colon@12..13 ":"
                          Operand@13..17
                            Literal@13..17
                              Boolean@13..17 "true"
                        Comma@17..18 ","
                        Clause@18..42
                          Property@18..34
                            Ident@18..22 "host"
                            Period@22..23 "."
                            Ident@23..34 "last_online"
                          Colon@34..35 ":"
                          Operand@35..42
                            Literal@35..42
                              String@35..42 "'today'"
                      RParen@42..43 ")"
                    Plus@43..44 "+"
                    Clause@44..73
                      Property@44..63
                        Ident@44..48 "host"
                        Period@48..49 "."
                        Ident@49..63 "sensor_version"
                      Colon@63..64 ":"
                      Operand@64..73
                        Literal@64..73
                          String@64..73 "'current'""#]],
        )
    }

    #[test]
    fn incomplete_parenthesized() {
        check(
            "(host.online:true,host.last_online)+host.sensor_version:'current'",
            expect![[r#"
                Root@0..65
                  InfixExpr@0..65
                    ParenExpr@0..35
                      LParen@0..1 "("
                      InfixExpr@1..34
                        Clause@1..17
                          Property@1..12
                            Ident@1..5 "host"
                            Period@5..6 "."
                            Ident@6..12 "online"
                          Colon@12..13 ":"
                          Operand@13..17
                            Literal@13..17
                              Boolean@13..17 "true"
                        Comma@17..18 ","
                        Clause@18..34
                          Property@18..34
                            Ident@18..22 "host"
                            Period@22..23 "."
                            Ident@23..34 "last_online"
                      RParen@34..35 ")"
                    Plus@35..36 "+"
                    Clause@36..65
                      Property@36..55
                        Ident@36..40 "host"
                        Period@40..41 "."
                        Ident@41..55 "sensor_version"
                      Colon@55..56 ":"
                      Operand@56..65
                        Literal@56..65
                          String@56..65 "'current'"

                At 34..35, expected '.', or ':', found ')'
                At 34..35, expected '!', '>', '<', '>=', '<=', '~', '!~', '[', boolean, string, or integer, found ')'"#]],
        );
    }

    #[test]
    fn bool_clause() {
        check(
            "host.online:true",
            expect![[r#"
                Root@0..16
                  Clause@0..16
                    Property@0..11
                      Ident@0..4 "host"
                      Period@4..5 "."
                      Ident@5..11 "online"
                    Colon@11..12 ":"
                    Operand@12..16
                      Literal@12..16
                        Boolean@12..16 "true""#]],
        );
    }

    #[test]
    fn missing_operand() {
        check(
            "host.last_online:",
            expect![[r#"
                Root@0..17
                  Clause@0..17
                    Property@0..16
                      Ident@0..4 "host"
                      Period@4..5 "."
                      Ident@5..16 "last_online"
                    Colon@16..17 ":"

                At 16..17, expected '!', '>', '<', '>=', '<=', '~', '!~', '[', boolean, string, or integer"#]],
        );
    }

    #[test]
    fn not_string() {
        check(
            "host.platform:!'Linux'",
            expect![[r#"
                Root@0..22
                  Clause@0..22
                    Property@0..13
                      Ident@0..4 "host"
                      Period@4..5 "."
                      Ident@5..13 "platform"
                    Colon@13..14 ":"
                    Operator@14..15
                      Bang@14..15 "!"
                    Operand@15..22
                      Literal@15..22
                        String@15..22 "'Linux'""#]],
        )
    }

    #[test]
    fn too_many_operators() {
        check(
            "host.online:><true",
            expect![[r#"
                Root@0..14
                  Clause@0..14
                    Property@0..11
                      Ident@0..4 "host"
                      Period@4..5 "."
                      Ident@5..11 "online"
                    Colon@11..12 ":"
                    Operator@12..13
                      Gt@12..13 ">"
                    Error@13..14
                      Lt@13..14 "<"

                At 13..14, expected '[', boolean, string, or integer, found '<'"#]],
        )
    }

    #[test]
    fn not_an_operator() {
        check(
            "host.online:?true",
            expect![[r#"
                Root@0..13
                  Clause@0..13
                    Property@0..11
                      Ident@0..4 "host"
                      Period@4..5 "."
                      Ident@5..11 "online"
                    Colon@11..12 ":"
                    Error@12..13
                      Error@12..13 "?"

                At 12..13, expected '!', '>', '<', '>=', '<=', '~', '!~', '[', boolean, string, or integer, found error"#]],
        )
    }

    #[test]
    fn not_exact_string() {
        check(
            "hostname:!['sample']",
            expect![[r#"
                Root@0..20
                  Clause@0..20
                    Property@0..8
                      Ident@0..8 "hostname"
                    Colon@8..9 ":"
                    Operator@9..10
                      Bang@9..10 "!"
                    Operand@10..20
                      LBracket@10..11 "["
                      Literal@11..19
                        String@11..19 "'sample'"
                      RBracket@19..20 "]""#]],
        )
    }

    #[test]
    fn gt_100() {
        check(
            "host.risk_score:>100",
            expect![[r#"
                Root@0..20
                  Clause@0..20
                    Property@0..15
                      Ident@0..4 "host"
                      Period@4..5 "."
                      Ident@5..15 "risk_score"
                    Colon@15..16 ":"
                    Operator@16..17
                      Gt@16..17 ">"
                    Operand@17..20
                      Literal@17..20
                        Integer@17..20 "100""#]],
        )
    }
}
