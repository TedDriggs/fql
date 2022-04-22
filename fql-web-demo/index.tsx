import { render } from "react-dom";
import init, { parse } from "fql-ts";
import React from "react";

// Wait for the page to load
window.addEventListener("load", () => {
    // Wait for the WASM module to start
    init().then(() => {
        // Wire up the demo
        document.getElementById("demo").addEventListener("input", e => {
            const result = parse((e.target as HTMLInputElement).value);
            console.info(result.expr?.asClause()?.property.toString());
            document.getElementById("output").innerText = result.debugTree();
            document.getElementById("error-output").innerText =
                result.diagnostics.map(d => d.message).join("\n") ||
                "NO ERRORS";
        });
    });
});

render(
    <React.StrictMode>
        <em>Hello React</em>
    </React.StrictMode>,
    document.getElementById("app")
);
