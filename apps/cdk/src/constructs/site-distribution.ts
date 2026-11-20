import { CfnOutput, Duration } from "aws-cdk-lib";
import { Certificate, CertificateValidation, type CfnCertificate } from "aws-cdk-lib/aws-certificatemanager";
import {
    AllowedMethods,
    CachedMethods,
    CachePolicy,
    Distribution,
    HttpVersion,
    OriginProtocolPolicy,
    PriceClass,
    ResponseHeadersPolicy,
    ViewerProtocolPolicy,
} from "aws-cdk-lib/aws-cloudfront";
import { HttpOrigin } from "aws-cdk-lib/aws-cloudfront-origins";
import { AaaaRecord, ARecord, HostedZone, RecordTarget, type CfnRecordSet } from "aws-cdk-lib/aws-route53";
import { CloudFrontTarget } from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";

/**
 * The logical IDs these two resources already have in the deployed stack, from before CloudFront. CDK derives a logical
 * ID from a resource's position in the construct tree, so moving them here would rename them, and CloudFormation would
 * then create a second A record before deleting the first, which Route 53 rejects. Pinning the IDs makes the existing
 * record update in place to point at CloudFront instead.
 */
const EXISTING_CERTIFICATE_LOGICAL_ID = "ApiCertificate285C31EB";
const EXISTING_ALIAS_RECORD_LOGICAL_ID = "ApiAlias58B2F8C6";

export interface SiteDistributionProps {
    readonly domainName: string;
    readonly hostedZoneName: string;
    /** The hostname CloudFront fetches from: the API's default endpoint. */
    readonly originDomainName: string;
}

/**
 * The public face of the site: a CloudFront distribution on the custom domain, with its certificate and DNS records.
 *
 * It redirects HTTP to HTTPS at the edge, terminates TLS near the client, serves HTTP/2 and HTTP/3, compresses, and caches
 * every response the API marks cacheable, so a hit never reaches API Gateway or the Lambda.
 */
export class SiteDistribution extends Construct {
    readonly distribution: Distribution;

    constructor(scope: Construct, id: string, props: SiteDistributionProps) {
        super(scope, id);

        const zone = HostedZone.fromLookup(this, "Zone", { domainName: props.hostedZoneName });
        const certificate = new Certificate(this, "Certificate", {
            domainName: props.domainName,
            validation: CertificateValidation.fromDns(zone),
        });
        (certificate.node.defaultChild as CfnCertificate).overrideLogicalId(EXISTING_CERTIFICATE_LOGICAL_ID);

        // The API decides what may be cached through Cache-Control; this policy only obeys it. With a default TTL of zero,
        // a response that says nothing (or `no-store`) is never cached. Nothing in the request varies the response, so no
        // cookie, header or query string is part of the cache key.
        const cachePolicy = new CachePolicy(this, "CachePolicy", {
            comment: "Obeys the origin's Cache-Control and caches nothing without it",
            minTtl: Duration.seconds(0),
            defaultTtl: Duration.seconds(0),
            maxTtl: Duration.days(1),
            enableAcceptEncodingGzip: true,
            enableAcceptEncodingBrotli: true,
        });

        // Not `includeSubdomains`: this header would otherwise commit every other subdomain of the zone to HTTPS.
        const responseHeadersPolicy = new ResponseHeadersPolicy(this, "ResponseHeadersPolicy", {
            comment: "HTTPS-only for browsers, and no content sniffing of the plain-text responses",
            securityHeadersBehavior: {
                strictTransportSecurity: {
                    accessControlMaxAge: Duration.days(365),
                    includeSubdomains: false,
                    preload: false,
                    override: true,
                },
                contentTypeOptions: { override: true },
            },
        });

        this.distribution = new Distribution(this, "Distribution", {
            comment: props.domainName,
            domainNames: [props.domainName],
            certificate,
            httpVersion: HttpVersion.HTTP2_AND_3,
            // North America and Europe, the cheapest set of edge locations.
            priceClass: PriceClass.PRICE_CLASS_100,
            defaultBehavior: {
                origin: new HttpOrigin(props.originDomainName, { protocolPolicy: OriginProtocolPolicy.HTTPS_ONLY }),
                viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
                allowedMethods: AllowedMethods.ALLOW_GET_HEAD,
                cachedMethods: CachedMethods.CACHE_GET_HEAD,
                cachePolicy,
                responseHeadersPolicy,
                compress: true,
            },
        });

        const target = RecordTarget.fromAlias(new CloudFrontTarget(this.distribution));
        const aRecord = new ARecord(this, "Alias", { zone, recordName: props.domainName, target });
        (aRecord.node.defaultChild as CfnRecordSet).overrideLogicalId(EXISTING_ALIAS_RECORD_LOGICAL_ID);
        new AaaaRecord(this, "AliasIpv6", { zone, recordName: props.domainName, target });

        new CfnOutput(this, "SiteUrl", { value: `https://${props.domainName}` });
    }
}
