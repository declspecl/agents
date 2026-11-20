import { Stage } from "../../model/stage";
import { SiteStack } from "../site-stack";
import { App } from "aws-cdk-lib";
import { Match, Template } from "aws-cdk-lib/assertions";
import { beforeAll, describe, expect, it } from "vitest";

/** The logical IDs the deployed stack already has. See `site-distribution.ts` for why they must not change. */
const EXISTING_CERTIFICATE_LOGICAL_ID = "ApiCertificate285C31EB";
const EXISTING_ALIAS_RECORD_LOGICAL_ID = "ApiAlias58B2F8C6";

let template: Template;

beforeAll(() => {
    // An empty list of stacks to bundle skips the Docker build, so the test needs no Docker and takes milliseconds.
    const app = new App({ context: { "aws:cdk:bundling-stacks": [] } });
    const stack = new SiteStack(app, "agents-prod-api", {
        env: { account: "123456789012", region: "us-east-1" },
        stage: Stage.PROD,
        domainName: "agents.example.com",
        hostedZoneName: "example.com",
    });
    template = Template.fromStack(stack);
});

function onlyResourceOfType(type: string): Record<string, unknown> {
    const found = Object.values(template.findResources(type));
    expect(found).toHaveLength(1);

    return found[0] as Record<string, unknown>;
}

describe("the CloudFront distribution", () => {
    it("serves the custom domain with the stack's certificate", () => {
        template.hasResourceProperties("AWS::CloudFront::Distribution", {
            DistributionConfig: Match.objectLike({
                Aliases: ["agents.example.com"],
                ViewerCertificate: Match.objectLike({
                    AcmCertificateArn: { Ref: EXISTING_CERTIFICATE_LOGICAL_ID },
                    MinimumProtocolVersion: "TLSv1.2_2021",
                }),
            }),
        });
    });

    it("redirects HTTP to HTTPS at the edge", () => {
        template.hasResourceProperties("AWS::CloudFront::Distribution", {
            DistributionConfig: Match.objectLike({ DefaultCacheBehavior: Match.objectLike({ ViewerProtocolPolicy: "redirect-to-https" }) }),
        });
    });

    it("allows only GET and HEAD, compresses, and speaks HTTP/3 over IPv6", () => {
        template.hasResourceProperties("AWS::CloudFront::Distribution", {
            DistributionConfig: Match.objectLike({
                DefaultCacheBehavior: Match.objectLike({ AllowedMethods: ["GET", "HEAD"], CachedMethods: ["GET", "HEAD"], Compress: true }),
                HttpVersion: "http2and3",
                IPV6Enabled: true,
            }),
        });
    });

    it("fetches from the API's default endpoint over HTTPS only", () => {
        template.hasResourceProperties("AWS::CloudFront::Distribution", {
            DistributionConfig: Match.objectLike({
                Origins: [
                    Match.objectLike({
                        DomainName: Match.anyValue(),
                        CustomOriginConfig: Match.objectLike({ OriginProtocolPolicy: "https-only" }),
                    }),
                ],
            }),
        });
        const origin = JSON.stringify(onlyResourceOfType("AWS::CloudFront::Distribution"));

        expect(origin).toContain("execute-api");
    });
});

describe("caching", () => {
    it("obeys the origin's Cache-Control and caches nothing without it", () => {
        template.hasResourceProperties("AWS::CloudFront::CachePolicy", {
            CachePolicyConfig: Match.objectLike({ MinTTL: 0, DefaultTTL: 0, MaxTTL: 86_400 }),
        });
    });

    it("varies the cache on nothing but the encoding", () => {
        template.hasResourceProperties("AWS::CloudFront::CachePolicy", {
            CachePolicyConfig: Match.objectLike({
                ParametersInCacheKeyAndForwardedToOrigin: Match.objectLike({
                    EnableAcceptEncodingGzip: true,
                    EnableAcceptEncodingBrotli: true,
                    CookiesConfig: { CookieBehavior: "none" },
                    HeadersConfig: { HeaderBehavior: "none" },
                    QueryStringsConfig: { QueryStringBehavior: "none" },
                }),
            }),
        });
    });
});

describe("response headers", () => {
    it("commits browsers to HTTPS for a year without touching other subdomains", () => {
        template.hasResourceProperties("AWS::CloudFront::ResponseHeadersPolicy", {
            ResponseHeadersPolicyConfig: Match.objectLike({
                SecurityHeadersConfig: Match.objectLike({
                    StrictTransportSecurity: {
                        AccessControlMaxAgeSec: 31_536_000,
                        IncludeSubdomains: false,
                        Override: true,
                        Preload: false,
                    },
                    ContentTypeOptions: { Override: true },
                }),
            }),
        });
    });
});

describe("DNS", () => {
    it("points the existing A record at CloudFront, in place", () => {
        const resources = template.toJSON().Resources as Record<string, { Type: string; Properties: Record<string, unknown> }>;
        const record = resources[EXISTING_ALIAS_RECORD_LOGICAL_ID];

        expect(record?.Type).toBe("AWS::Route53::RecordSet");
        expect(record?.Properties).toMatchObject({ Type: "A", Name: "agents.example.com." });
        expect(JSON.stringify(record?.Properties["AliasTarget"])).toContain("Distribution");
    });

    it("adds an IPv6 record pointing at CloudFront", () => {
        template.hasResourceProperties("AWS::Route53::RecordSet", { Type: "AAAA", Name: "agents.example.com." });
    });

    it("keeps the existing certificate's logical ID, so it is not replaced", () => {
        const resources = template.toJSON().Resources as Record<string, { Type: string }>;

        expect(resources[EXISTING_CERTIFICATE_LOGICAL_ID]?.Type).toBe("AWS::CertificateManager::Certificate");
        template.resourceCountIs("AWS::CertificateManager::Certificate", 1);
    });
});

describe("the API", () => {
    it("no longer has a custom domain of its own", () => {
        template.resourceCountIs("AWS::ApiGatewayV2::DomainName", 0);
        template.resourceCountIs("AWS::ApiGatewayV2::ApiMapping", 0);
    });

    it("is a zipped provided.al2023 ARM64 function with the smallest memory and a short timeout", () => {
        template.hasResourceProperties("AWS::Lambda::Function", {
            Runtime: "provided.al2023",
            Handler: "bootstrap",
            Architectures: ["arm64"],
            MemorySize: 128,
            Timeout: 3,
        });
    });

    it("has no hard-coded function name and no container image", () => {
        const properties = onlyResourceOfType("AWS::Lambda::Function")["Properties"] as Record<string, unknown>;

        expect(properties["FunctionName"]).toBeUndefined();
        expect(properties["PackageType"]).toBeUndefined();
    });

    it("tells the function its public origin", () => {
        template.hasResourceProperties("AWS::Lambda::Function", {
            Environment: { Variables: Match.objectLike({ PUBLIC_BASE_URL: "https://agents.example.com" }) },
        });
    });

    it("keeps the log group's name, since renaming a named log group replaces it", () => {
        template.hasResourceProperties("AWS::Logs::LogGroup", { LogGroupName: "/aws/lambda/agents-prod-api" });
    });
});
