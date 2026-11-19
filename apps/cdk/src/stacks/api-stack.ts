import { Api } from "../constructs/api";
import type { Stage } from "../model/stage";
import { Stack, type StackProps } from "aws-cdk-lib";
import type { Construct } from "constructs";

export interface ApiStackProps extends StackProps {
    readonly stage: Stage;
    readonly domainName: string;
    readonly hostedZoneName: string;
}

export class ApiStack extends Stack {
    readonly api: Api;

    constructor(scope: Construct, id: string, props: ApiStackProps) {
        super(scope, id, props);

        this.api = new Api(this, "Api", props);
    }
}
