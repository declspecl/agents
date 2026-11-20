import { LAMBDA_ENVIRONMENT_VARIABLE } from "../environment/lambda-variables";
import type { Stage } from "../model/stage";
import { RustLambda } from "./rust-lambda";
import { Stack } from "aws-cdk-lib";
import { HttpApi, HttpMethod } from "aws-cdk-lib/aws-apigatewayv2";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Construct } from "constructs";

export interface ApiProps {
    readonly stage: Stage;
    /** The public hostname the site is served on, which the API uses for absolute links. */
    readonly domainName: string;
}

/**
 * The Rust API: one Lambda behind an HTTP API. It has no custom domain of its own. CloudFront serves the public
 * hostname and reaches this API at its default endpoint.
 */
export class Api extends Construct {
    readonly api: HttpApi;
    readonly apiFunction: RustLambda;
    /** The hostname of the API's default endpoint, for CloudFront to use as its origin. */
    readonly originDomainName: string;

    constructor(scope: Construct, id: string, props: ApiProps) {
        super(scope, id);

        this.api = new HttpApi(this, "HttpApi", { apiName: `agents-${props.stage}-api` });

        this.apiFunction = new RustLambda(this, "ApiFunction", {
            crate: "api",
            binary: "api",
            // Kept at the name the first deploy gave it: renaming a named log group replaces it, which CloudFormation refuses.
            logGroupName: `/aws/lambda/agents-${props.stage}-api`,
            environment: {
                TZ: ":UTC",
                [LAMBDA_ENVIRONMENT_VARIABLE.publicBaseUrl]: `https://${props.domainName}`,
            },
        });

        this.api.addRoutes({
            path: "/{proxy+}",
            methods: [HttpMethod.ANY],
            integration: new HttpLambdaIntegration("ApiIntegration", this.apiFunction),
        });

        const stack = Stack.of(this);
        this.originDomainName = `${this.api.apiId}.execute-api.${stack.region}.${stack.urlSuffix}`;
    }
}
