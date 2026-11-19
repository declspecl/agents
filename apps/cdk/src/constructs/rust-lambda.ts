import { resolveRepositoryPath } from "../utils/paths";
import { Duration, RemovalPolicy } from "aws-cdk-lib";
import { Platform } from "aws-cdk-lib/aws-ecr-assets";
import { Architecture, DockerImageCode, DockerImageFunction, LoggingFormat, type DockerImageFunctionProps } from "aws-cdk-lib/aws-lambda";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import type { Construct } from "constructs";

export interface RustLambdaProps extends Omit<DockerImageFunctionProps, "code" | "architecture" | "logGroup"> {
    readonly functionName: string;
    /** Crate directory under apps/ containing the Dockerfile. */
    readonly crate: string;
}

/** An ARM64 container-image function. Docker compiles the Rust crate, so no native toolchain is needed to deploy. */
export class RustLambda extends DockerImageFunction {
    constructor(scope: Construct, id: string, { crate, ...props }: RustLambdaProps) {
        super(scope, id, {
            timeout: Duration.seconds(10),
            memorySize: 256,
            ...props,
            architecture: Architecture.ARM_64,
            code: DockerImageCode.fromImageAsset(resolveRepositoryPath("apps", crate), { platform: Platform.LINUX_ARM64 }),
            loggingFormat: LoggingFormat.JSON,
            logGroup: new LogGroup(scope, `${id}LogGroup`, {
                logGroupName: `/aws/lambda/${props.functionName}`,
                retention: RetentionDays.ONE_MONTH,
                removalPolicy: RemovalPolicy.DESTROY,
            }),
        });
    }
}
