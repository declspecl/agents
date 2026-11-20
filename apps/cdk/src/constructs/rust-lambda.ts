import { resolveRepositoryPath } from "../utils/paths";
import { AssetHashType, DockerImage, Duration, RemovalPolicy } from "aws-cdk-lib";
import { Architecture, Code, Function, LoggingFormat, Runtime, type FunctionProps } from "aws-cdk-lib/aws-lambda";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import type { Construct } from "constructs";

export interface RustLambdaProps extends Omit<
    FunctionProps,
    "code" | "handler" | "runtime" | "architecture" | "logGroup" | "functionName"
> {
    /** Crate directory under apps/, which holds the Cargo.toml and `builder.Dockerfile`. */
    readonly crate: string;
    /** The binary the crate builds, which becomes the function's `bootstrap`. */
    readonly binary: string;
    readonly logGroupName: string;
}

/** Source that does not change the binary, so editing it does not rebuild or redeploy the function. */
const IGNORED_SOURCE = ["target", "tests", "node_modules", ".turbo", ".cache", "package.json", "*.md"];

/**
 * An ARM64 `provided.al2023` function running a plain binary. Docker compiles the crate inside `builder.Dockerfile`, so
 * a deploy needs Docker and no Rust or cross-compilation toolchain. Only the resulting `bootstrap` is zipped and uploaded.
 *
 * The function name is left for CloudFormation to generate, since a hard-coded name blocks any change that replaces the
 * function.
 */
export class RustLambda extends Function {
    constructor(scope: Construct, id: string, { crate, binary, logGroupName, ...props }: RustLambdaProps) {
        const crateDirectory = resolveRepositoryPath("apps", crate);

        super(scope, id, {
            // Sized to what the function uses, not to a comfortable default. The process holds about 4 MB and each
            // request is a few microseconds of in-memory work, so the smallest memory Lambda offers and a short
            // timeout are enough. A request that is still running after three seconds is stuck.
            timeout: Duration.seconds(3),
            memorySize: 128,
            ...props,
            runtime: Runtime.PROVIDED_AL2023,
            architecture: Architecture.ARM_64,
            handler: "bootstrap",
            code: Code.fromAsset(crateDirectory, {
                exclude: IGNORED_SOURCE,
                // Hash the source, not the build output, so an unchanged crate is not rebuilt on every synth.
                assetHashType: AssetHashType.SOURCE,
                bundling: {
                    image: DockerImage.fromBuild(crateDirectory, { file: "builder.Dockerfile", platform: "linux/arm64" }),
                    platform: "linux/arm64",
                    user: "root",
                    environment: { CARGO_TARGET_DIR: "/tmp/target" },
                    command: [
                        "bash",
                        "-c",
                        `cargo build --release --locked && cp "$CARGO_TARGET_DIR/release/${binary}" /asset-output/bootstrap`,
                    ],
                },
            }),
            loggingFormat: LoggingFormat.JSON,
            logGroup: new LogGroup(scope, `${id}LogGroup`, {
                logGroupName,
                retention: RetentionDays.ONE_MONTH,
                removalPolicy: RemovalPolicy.DESTROY,
            }),
        });
    }
}
