import { configDefaults, defineConfig } from "vitest/config";

export default defineConfig({
    test: {
        // `tsc` also emits compiled copies of the tests into dist/, and cdk.out/ holds bundled assets; neither is source.
        exclude: [...configDefaults.exclude, "dist/**", "cdk.out/**"],
    },
});
