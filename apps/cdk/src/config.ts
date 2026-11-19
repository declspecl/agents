import { Stage } from "./model/stage";

export interface DeploymentConfig {
    readonly stage: Stage;
    readonly domainName: string;
    readonly hostedZoneName: string;
    readonly env: {
        readonly account: string;
        readonly region: string;
    };
}

export type Environment = Readonly<Record<string, string | undefined>>;

export function readDeploymentConfig(environment: Environment): DeploymentConfig {
    return {
        stage: Stage.PROD,
        domainName: optional(environment, "DOMAIN_NAME") ?? "agents.gavindhondt.com",
        hostedZoneName: optional(environment, "HOSTED_ZONE_NAME") ?? "gavindhondt.com",
        env: {
            account: require(environment, "AWS_ACCOUNT_ID"),
            region: optional(environment, "AWS_REGION") ?? "us-east-1",
        },
    };
}

function optional(environment: Environment, name: string): string | undefined {
    const value = environment[name]?.trim();

    return value || undefined;
}

function require(environment: Environment, name: string): string {
    const value = optional(environment, name);
    if (value === undefined) {
        throw new Error(`${name} must be set`);
    }

    return value;
}
