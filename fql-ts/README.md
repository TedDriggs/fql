# fql-ts

TypeScript bindings for `fql` parser.

# Build Instructions

1. Run `cargo install wasm-pack`
2. Run `wasm-pack build --target web`

# Output Snapshot

Note that the `free` methods are added by `wasm-bindgen`.

```typescript
/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} input
 * @returns {Parse}
 */
export function parse(input: string): Parse;

/**
 * The value of a literal.
 */
export type LitValue = string | number | boolean;

/**
 * A single property, operator, and operand, such as `online:true`.
 */
export class Clause {
    free(): void;
    /**
     * @returns {Operand | undefined}
     */
    readonly operand: Operand | undefined;
    /**
     * @returns {Property | undefined}
     */
    readonly property: Property | undefined;
}
/**
 */
export class Diagnostic {
    free(): void;
    /**
     */
    readonly message: void;
}
/**
 */
export class Expr {
    free(): void;
    /**
     * If the expression is a binary (infix) expression, get it and narrow the type.
     * @returns {ExprBinary | undefined}
     */
    asBinary(): ExprBinary | undefined;
    /**
     * @returns {Clause | undefined}
     */
    asClause(): Clause | undefined;
    /**
     * @returns {ExprParen | undefined}
     */
    asParen(): ExprParen | undefined;
}
/**
 * A binary expression, such as `os:'windows'+online:true`.
 */
export class ExprBinary {
    free(): void;
    /**
     * @returns {Expr | undefined}
     */
    readonly lhs: Expr | undefined;
    /**
     * @returns {Expr | undefined}
     */
    readonly rhs: Expr | undefined;
}
/**
 */
export class ExprParen {
    free(): void;
    /**
     * @returns {Expr | undefined}
     */
    readonly body: Expr | undefined;
}
/**
 * A literal value, such as `true`, `5`, or `'falcon'`.
 */
export class Literal {
    free(): void;
    /**
     * The value of a literal.
     *
     * This can be `undefined` in case of invalid input; because the parser is
     * fault-tolerant, it will still produce a parse result.
     * @returns {LitValue | undefined}
     */
    readonly value: LitValue | undefined;
}
/**
 */
export class Operand {
    free(): void;
    /**
     * The literal value in the operand.
     * @returns {Literal | undefined}
     */
    literal(): Literal | undefined;
}
/**
 */
export class Parse {
    free(): void;
    /**
     * Generate a string debug representation of the parse tree.
     * @returns {string}
     */
    debugTree(): string;
    /**
     * A list of diagnostics pertaining to the parse result.
     * @returns {any[]}
     */
    readonly diagnostics: any[];
    /**
     * Get the expression produced by the parsing. This can be `None` if the parser
     * was unable to find any fragment of an expression in the input.
     * @returns {Expr | undefined}
     */
    readonly expr: Expr | undefined;
}
/**
 */
export class Property {
    free(): void;
    /**
     * @returns {string}
     */
    toString(): string;
}
```
