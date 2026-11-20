import { Api } from "../constructs/api";
import { SiteDistribution } from "../constructs/site-distribution";
import type { Stage } from "../model/stage";
import { Stack, type StackProps } from "aws-cdk-lib";
import type { Construct } from "constructs";

export interface SiteStackProps extends StackProps {
    readonly stage: Stage;
    readonly domainName: string;
    readonly hostedZoneName: string;
}

/** The whole site: the Rust API, and the CloudFront distribution and DNS that serve it. */
export class SiteStack extends Stack {
    readonly api: Api;
    readonly site: SiteDistribution;

    constructor(scope: Construct, id: string, props: SiteStackProps) {
        super(scope, id, props);

        this.api = new Api(this, "Api", { stage: props.stage, domainName: props.domainName });
        this.site = new SiteDistribution(this, "Site", {
            domainName: props.domainName,
            hostedZoneName: props.hostedZoneName,
            originDomainName: this.api.originDomainName,
        });
    }
}
