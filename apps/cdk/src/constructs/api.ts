import type { Stage } from "../model/stage";
import { RustLambda } from "./rust-lambda";
import { CfnOutput } from "aws-cdk-lib";
import { DomainName, HttpApi, HttpMethod } from "aws-cdk-lib/aws-apigatewayv2";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Certificate, CertificateValidation } from "aws-cdk-lib/aws-certificatemanager";
import { ARecord, HostedZone, RecordTarget } from "aws-cdk-lib/aws-route53";
import { ApiGatewayv2DomainProperties } from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";

export interface ApiProps {
    readonly stage: Stage;
    readonly domainName: string;
    readonly hostedZoneName: string;
}

/** The Rust REST API: one Lambda behind an HTTP API, served on a custom domain. */
export class Api extends Construct {
    readonly api: HttpApi;
    readonly apiFunction: RustLambda;

    constructor(scope: Construct, id: string, props: ApiProps) {
        super(scope, id);

        const zone = HostedZone.fromLookup(this, "Zone", { domainName: props.hostedZoneName });
        const certificate = new Certificate(this, "Certificate", {
            domainName: props.domainName,
            validation: CertificateValidation.fromDns(zone),
        });
        const domainName = new DomainName(this, "DomainName", { domainName: props.domainName, certificate });

        this.api = new HttpApi(this, "HttpApi", {
            apiName: `agents-${props.stage}-api`,
            defaultDomainMapping: { domainName },
        });

        this.apiFunction = new RustLambda(this, "ApiFunction", {
            functionName: `agents-${props.stage}-api`,
            crate: "api",
            environment: { TZ: ":UTC" },
        });

        this.api.addRoutes({
            path: "/{proxy+}",
            methods: [HttpMethod.ANY],
            integration: new HttpLambdaIntegration("ApiIntegration", this.apiFunction),
        });

        new ARecord(this, "Alias", {
            zone,
            recordName: props.domainName,
            target: RecordTarget.fromAlias(
                new ApiGatewayv2DomainProperties(domainName.regionalDomainName, domainName.regionalHostedZoneId),
            ),
        });

        new CfnOutput(this, "ApiUrl", { value: `https://${props.domainName}` });
    }
}
