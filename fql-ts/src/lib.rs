use fql::{ast, Spanned};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn parse(input: &str) -> Parse {
    Parse(fql::parse(input))
}

#[wasm_bindgen]
pub struct Parse(fql::Parse);

#[wasm_bindgen]
impl Parse {
    /// Get the expression produced by the parsing. This can be `None` if the parser
    /// was unable to find any fragment of an expression in the input.
    #[wasm_bindgen(getter)]
    pub fn expr(&self) -> Option<Expr> {
        self.0.to_expr().map(Expr)
    }

    /// Generate a string debug representation of the parse tree.
    #[wasm_bindgen(js_name = "debugTree")]
    pub fn debug_tree(&self) -> String {
        self.0.debug_tree()
    }

    /// A list of diagnostics pertaining to the parse result.
    #[wasm_bindgen(getter)]
    pub fn diagnostics(&self) -> Vec<JsValue> {
        // This is very ugly, but wasm_bindgen doesn't support returning a `Vec` of anything other
        // than `JsValue`.
        self.0
            .diagnostics()
            .map(Diagnostic::from)
            .map(JsValue::from)
            .collect::<Vec<JsValue>>()
    }
}

#[wasm_bindgen]
pub struct Expr(ast::Expr);

#[wasm_bindgen]
impl Expr {
    /// If the expression is a binary (infix) expression, get it and narrow the type.
    #[wasm_bindgen(js_name = "asBinary")]
    pub fn as_binary(&self) -> Option<ExprBinary> {
        if let ast::Expr::Binary(v) = &self.0 {
            Some(v.clone()).map(ExprBinary)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = "asClause")]
    pub fn as_clause(&self) -> Option<Clause> {
        if let ast::Expr::Clause(v) = &self.0 {
            Some(v.clone()).map(Clause)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = "asParen")]
    pub fn as_paren(&self) -> Option<ExprParen> {
        if let ast::Expr::Paren(v) = &self.0 {
            Some(v.clone()).map(ExprParen)
        } else {
            None
        }
    }
}

/// A binary expression, such as `os:'windows'+online:true`.
#[wasm_bindgen]
pub struct ExprBinary(ast::ExprBinary);

#[wasm_bindgen]
impl ExprBinary {
    #[wasm_bindgen(getter)]
    pub fn lhs(&self) -> Option<Expr> {
        self.0.lhs().map(Expr)
    }

    #[wasm_bindgen(getter)]
    pub fn rhs(&self) -> Option<Expr> {
        self.0.rhs().map(Expr)
    }
}

/// A single property, operator, and operand, such as `online:true`.
#[wasm_bindgen]
pub struct Clause(ast::Clause);

#[wasm_bindgen]
impl Clause {
    #[wasm_bindgen(getter)]
    pub fn property(&self) -> Option<Property> {
        self.0.property().map(Property)
    }

    #[wasm_bindgen(getter)]
    pub fn operand(&self) -> Option<Operand> {
        self.0.operand().map(Operand)
    }
}

#[wasm_bindgen]
pub struct ExprParen(ast::ExprParen);

#[wasm_bindgen]
impl ExprParen {
    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Option<Expr> {
        self.0.body().map(Expr)
    }
}

#[wasm_bindgen]
pub struct Property(ast::Property);

#[wasm_bindgen]
impl Property {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[wasm_bindgen]
pub struct Operand(ast::Operand);

#[wasm_bindgen]
impl Operand {
    /// The literal value in the operand.
    pub fn literal(&self) -> Option<Literal> {
        self.0.literal().map(Literal)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const LIT_VALUE: &'static str = r#"
/**
 * The value of a literal.
 */
export type LitValue = string | number | boolean;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "LitValue | undefined")]
    pub type LitValueOrUndefined;
}

/// A literal value, such as `true`, `5`, or `'falcon'`.
#[wasm_bindgen]
pub struct Literal(ast::Literal);

#[wasm_bindgen]
impl Literal {
    /// The value of a literal.
    ///
    /// This can be `undefined` in case of invalid input; because the parser is
    /// fault-tolerant, it will still produce a parse result.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> LitValueOrUndefined {
        self.0
            .value()
            .map(|v| match v {
                ast::Lit::Str(s) => JsValue::from_str(&s.value()),
                ast::Lit::Bool(b) => JsValue::from_bool(b.value()),
                ast::Lit::Int(i) => match i.value() {
                    Ok(v) => JsValue::from_f64(v as f64),
                    Err(_) => JsValue::from_f64(f64::NAN),
                },
            })
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into()
    }
}

// This doesn't use wasm_bindgen(getter_with_clone) due to known issue
// with `readonly` https://github.com/rustwasm/wasm-bindgen/issues/2721
#[wasm_bindgen]
pub struct Diagnostic {
    message: String,
    range: TextRange,
}

#[wasm_bindgen]
impl Diagnostic {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn range(&self) -> TextRange {
        self.range.clone()
    }
}

impl<'a> From<&'a fql::ParseError> for Diagnostic {
    fn from(e: &'a fql::ParseError) -> Self {
        Self {
            message: format!("{e:#}"),
            range: TextRange(e.span()),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TextRange(fql::TextRange);

#[wasm_bindgen]
impl TextRange {
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> f64 {
        usize::from(self.0.start()) as f64
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> f64 {
        usize::from(self.0.end()) as f64
    }

    #[wasm_bindgen(getter)]
    pub fn length(&self) -> f64 {
        usize::from(self.0.len()) as f64
    }
}
