import jseslint from "@eslint/js";
import eslintConfigPrettier from "eslint-config-prettier";
import turboPlugin from "eslint-plugin-turbo";
import eslintPluginUnicorn from "eslint-plugin-unicorn";
import { defineConfig, globalIgnores } from "eslint/config";
import tseslint from "typescript-eslint";

/** @type {import("eslint").Linter.ConfigWithExtendsArray} */
export const options = [
    jseslint.configs.recommended,
    tseslint.configs.recommended,
    eslintPluginUnicorn.configs.recommended,
    globalIgnores([".husky", "node_modules", ".env*", "pnpm-*", "cdk.out/**", "dist/**", "target/**"]),
    {
        rules: {
            "unicorn/no-null": "off",
            "unicorn/no-array-reduce": "off",
            "unicorn/no-useless-undefined": "off",
            "unicorn/prevent-abbreviations": "off",
            "unicorn/no-for-loop": "error",
            // `export {}` turns a .d.ts file into a module so `declare global` augments the global scope.
            "unicorn/require-module-specifiers": "off",

            "@typescript-eslint/no-unused-vars": "warn",
            "@typescript-eslint/no-explicit-any": "error",
            "@typescript-eslint/consistent-type-imports": "error",

            curly: ["error", "all"],
            "padding-line-between-statements": ["error", { blankLine: "always", prev: "*", next: "return" }],
            "no-restricted-syntax": [
                "error",
                {
                    selector: "TSAsExpression[typeAnnotation.type='TSUnknownKeyword']",
                    message: "Do not use 'as unknown'. Fix the types strictly.",
                },
                {
                    selector: "TSAsExpression > TSAsExpression",
                    message: "Do not use double assertions. Fix the types strictly.",
                },
            ],
        },
    },
    { plugins: { turbo: turboPlugin }, rules: { "turbo/no-undeclared-env-vars": "warn" } },
    eslintConfigPrettier,
];

/** @type {import("eslint").Linter.Config} */
export default defineConfig(options);
