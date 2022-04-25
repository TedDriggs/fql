import init, { Diagnostic, parse } from "fql-ts";
import React, { FC, useEffect, useMemo, useState } from "react";

export const App: FC = () => {
    const [isLoaded, setLoaded] = useState(false);
    const fqlTs = useEffect(() => {
        init().then(() => setLoaded(true));
    }, []);
    return isLoaded ? <AppBody /> : <span>Loading...</span>;
};

const AppBody: FC = () => {
    const [input, setInput] = useState("");
    const result = useMemo(() => parse(input), [input]);

    return (
        <div className="demo-frame">
            <input
                id="demo"
                value={input}
                onInput={e => setInput((e.target as HTMLInputElement).value)}
            />
            <div id="error-output">
                {(result.diagnostics as Diagnostic[]).map((d, i) => (
                    <li key={i}>
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
