/**
 * The environment variables the CDK app sets on the API Lambda. Each name must match an entry in the Rust registry,
 * `Variable` in apps/api/src/environment/variable.rs, which a test enforces. This is separate from
 * `ENVIRONMENT_VARIABLE`, which lists what the CDK app itself reads.
 */
export const LAMBDA_ENVIRONMENT_VARIABLE = {
    /** The public origin the site is served from. Required by the API inside Lambda. */
    publicBaseUrl: "PUBLIC_BASE_URL",
} as const;
