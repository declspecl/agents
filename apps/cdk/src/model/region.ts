/**
 * The only region the stack deploys to. CloudFront serves the site, and a certificate for CloudFront must be issued in
 * us-east-1, so the stack, its certificate and its Lambda all live there.
 */
export enum AwsRegion {
    US_EAST_1 = "us-east-1",
}
