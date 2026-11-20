import { LAMBDA_ENVIRONMENT_VARIABLE } from "../lambda-variables";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

/** The names in the Rust `Variable::as_str` match arms: `Variable::X => "NAME",`. */
function readRustVariableNames(): string[] {
    const source = readFileSync(new URL("../../../../api/src/environment/variable.rs", import.meta.url), "utf8");

    return [...source.matchAll(/Variable::\w+ => "([A-Z][A-Z0-9_]+)"/g)]
        .map((match) => match[1])
        .filter((name): name is string => name !== undefined);
}

describe("LAMBDA_ENVIRONMENT_VARIABLE", () => {
    it("reads the Rust registry", () => {
        expect(readRustVariableNames()).toContain("PUBLIC_BASE_URL");
    });

    it("sets only variables the API reads", () => {
        const known = readRustVariableNames();

        for (const name of Object.values(LAMBDA_ENVIRONMENT_VARIABLE)) {
            expect(known, `${name} is not in the Rust Variable registry`).toContain(name);
        }
    });
});
