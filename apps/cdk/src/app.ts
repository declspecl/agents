#!/usr/bin/env node

import { readDeploymentConfig } from "./config";
import { ApiStack } from "./stacks/api-stack";
import { App } from "aws-cdk-lib";

const app = new App();
const config = readDeploymentConfig(process.env);

new ApiStack(app, `agents-${config.stage}-api`, {
    env: config.env,
    stage: config.stage,
    domainName: config.domainName,
    hostedZoneName: config.hostedZoneName,
});
