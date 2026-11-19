import baseConfig from "@agents/eslint/base";

export default [...baseConfig, { ignores: ["cdk.out/**", "dist/**"] }];
