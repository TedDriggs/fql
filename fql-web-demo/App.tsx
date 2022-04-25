import init, { Diagnostic, parse } from "fql-ts";
import React, { FC, useEffect, useMemo, useRef, useState } from "react";

export const App: FC = () => {
    const [isLoaded, setLoaded] = useState(false);
    const fqlTs = useEffect(() => {
        init().then(() => setLoaded(true));
    }, []);
    return isLoaded ? <AppBody /> : <span>Loading...</span>;
};

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
            <div id="output" style={{ whiteSpace: "pre" }}>
                {result.debugTree()}
            </div>
        </div>
    );
};
