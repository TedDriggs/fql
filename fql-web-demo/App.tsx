import init, { Diagnostic, parse } from "fql-ts";
import React, { FC, useMemo, useRef, useState } from "react";
import { RemoteSuspense, useAsyncOperation } from "ts-remote-data-react";

export const App: FC = () => (
    <RemoteSuspense
        data={useAsyncOperation(() => init(), [])}
        failureFallback={<strong>Unable to load WASM</strong>}
        loadingFallback={<>Loading...</>}
    >
        {() => <AppBody />}
    </RemoteSuspense>
);

const AppBody: FC = () => {
    const [inputText, setInput] = useState("");
    const inputElement = useRef<HTMLInputElement>(null);
    const result = useMemo(() => parse(inputText), [inputText]);

    return (
        <div className="demo-frame">
            <input
                id="demo"
                ref={inputElement}
                value={inputText}
                placeholder="Enter an FQL Expression"
                onInput={e => setInput((e.target as HTMLInputElement).value)}
            />
            <div id="error-output">
                {(result.diagnostics as Diagnostic[]).map((d, i) => (
                    <li
                        key={i}
                        onDoubleClick={e => {
                            e.stopPropagation();
                            e.preventDefault();
                            inputElement.current?.setSelectionRange(
                                d.range.start,
                                d.range.end,
                                "forward"
                            );
                        }}
                    >
                        {d.message} [Char {d.range.start.toLocaleString()}]
                    </li>
                ))}
            </div>
            <div id="output" style={{ whiteSpace: "pre", overflow: "auto" }}>
                {result.debugTree()}
            </div>
        </div>
    );
};
