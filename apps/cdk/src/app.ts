#!/usr/bin/env node

import { readDeploymentConfig } from "./config";
import { SiteStack } from "./stacks/site-stack";
import { App } from "aws-cdk-lib";

const app = new App();
const config = readDeploymentConfig(process.env);

// The stack keeps the id it was first deployed under, `-api`: renaming a stack deploys a second one.
new SiteStack(app, `agents-${config.stage}-api`, {
    env: config.env,
    stage: config.stage,
    domainName: config.domainName,
    hostedZoneName: config.hostedZoneName,
});
