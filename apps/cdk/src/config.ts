import { readEnvironment } from "./environment/read-environment";
import type { AwsAccountId, DomainName, HostedZoneName } from "./environment/schema";
import { ENVIRONMENT_VARIABLE, type RuntimeEnvironment } from "./environment/variables";
import type { AwsRegion } from "./model/region";
import type { Stage } from "./model/stage";

export interface DeploymentConfig {
    readonly stage: Stage;
    readonly domainName: DomainName;
    readonly hostedZoneName: HostedZoneName;
    readonly env: {
        readonly account: AwsAccountId;
        readonly region: AwsRegion;
    };
}

export function readDeploymentConfig(environment: RuntimeEnvironment): DeploymentConfig {
    const values = readEnvironment(environment);

    return {
        stage: values[ENVIRONMENT_VARIABLE.deploymentStage],
        domainName: values[ENVIRONMENT_VARIABLE.domainName],
        hostedZoneName: values[ENVIRONMENT_VARIABLE.hostedZoneName],
        env: {
            account: values[ENVIRONMENT_VARIABLE.awsAccountId],
            region: values[ENVIRONMENT_VARIABLE.awsRegion],
        },
    };
}
