---
status: accepted
---

# CloudFront serves the domain, redirects HTTP, and caches at the edge

`agents.gavindhondt.com` is a CloudFront distribution that fetches from the API Gateway HTTP API's default endpoint. It
replaces the API Gateway custom domain. It gives the site three things the custom domain could not:

- **HTTP works.** An API Gateway custom domain only listens on port 443, so `http://` hung. CloudFront listens on 80 and
  redirects to HTTPS at the edge.
- **Lower latency.** TCP and TLS end at a nearby edge instead of `us-east-1`, with HTTP/2 and HTTP/3, over IPv4 and IPv6.
- **Caching.** The API marks every successful response `Cache-Control: public, max-age=300`. The cache policy only obeys
  it (a default TTL of zero caches nothing without it), so a hit is served from the edge and never reaches API Gateway
  or the Lambda.

Browsers also get `Strict-Transport-Security` for a year, without `includeSubdomains`, which would commit every other
subdomain of the zone to HTTPS.

## Considered options

- **Leave it on the API Gateway custom domain.** Simplest, but HTTP hangs and every request pays the round trip to
  `us-east-1` and the Lambda.
- **A load balancer with a port 80 redirect.** Fixes HTTP only, and costs far more than the site itself.

## Consequences

- **Cost.** CloudFront's permanent free allowance (1 TB and 10 million requests per month, per AWS's pricing page) covers
  this site many times over, and cache hits skip API Gateway and Lambda charges. The price class is the cheapest
  (North America and Europe).
- **Region.** The stack is pinned to `us-east-1`, because a CloudFront certificate must be issued there. `AwsRegion` has
  that one member.
- **Staleness.** There is no invalidation step. After a deploy, the edge can serve the old copy for up to five minutes,
  and a browser that fetched it just before can keep it for five more.
- **The API's default endpoint stays reachable directly.** It bypasses the cache and the redirect. Everything is public,
  so this is harmless, but it means CloudFront is not a security boundary.
- **Cache key.** Nothing in the request varies the response, so the key is only the path and the encoding. If a response
  ever varies by `Accept` (JSON, planned in `docs/CONTENT.md`), add that header to the cache policy.
- **Logical IDs.** The certificate and the A record keep the IDs they had under the API construct, pinned in
  `site-distribution.ts`, and tests guard them. Moving them would rename them, and CloudFormation would create a second
  A record before deleting the first, which Route 53 rejects. The A record updates in place to point at CloudFront.
- **Deploy time.** A new distribution takes several minutes to deploy. The old path keeps serving until DNS switches.
