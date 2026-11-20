/**
 * Every environment variable the CDK app reads, and the only place its name is spelled. `.env.example` lists exactly
 * these, which a test enforces.
 */
export const ENVIRONMENT_VARIABLE = {
    /** The twelve-digit account to deploy into. Required: the hosted-zone lookup needs a concrete account at synth time. */
    awsAccountId: "AWS_ACCOUNT_ID",
    /** The region to deploy into. Defaults to us-east-1. */
    awsRegion: "AWS_REGION",
    /** The deployment stage. Only prod exists, so it defaults to prod. */
    deploymentStage: "DEPLOYMENT_STAGE",
    /** The public hostname the API is served on. Defaults to agents.gavindhondt.com. */
    domainName: "DOMAIN_NAME",
    /** The Route 53 hosted zone that contains the domain name. Defaults to gavindhondt.com. */
    hostedZoneName: "HOSTED_ZONE_NAME",
} as const;

export type EnvironmentVariableName = (typeof ENVIRONMENT_VARIABLE)[keyof typeof ENVIRONMENT_VARIABLE];

/** A process environment: what `process.env` is, and what tests pass in its place. */
export type RuntimeEnvironment = Readonly<Record<string, string | undefined>>;
