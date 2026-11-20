import { readDeploymentConfig } from "../../config";
import { readEnvironment } from "../read-environment";
import { ENVIRONMENT_VARIABLE } from "../variables";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const account = "123456789012";

describe("readDeploymentConfig", () => {
    it("defaults everything but the account", () => {
        const config = readDeploymentConfig({ AWS_ACCOUNT_ID: account });

        expect(config.stage).toBe("prod");
        expect(config.domainName).toBe("agents.gavindhondt.com");
        expect(config.hostedZoneName).toBe("gavindhondt.com");
        expect(config.env).toEqual({ account, region: "us-east-1" });
    });

    it("treats blank values as missing, as .env.example leaves them", () => {
        const config = readDeploymentConfig({ AWS_ACCOUNT_ID: account, AWS_REGION: "", DOMAIN_NAME: "  " });

        expect(config.env.region).toBe("us-east-1");
        expect(config.domainName).toBe("agents.gavindhondt.com");
    });

    it("normalises hostname case", () => {
        expect(readDeploymentConfig({ AWS_ACCOUNT_ID: account, DOMAIN_NAME: "API.GavinDhondt.com" }).domainName).toBe(
            "api.gavindhondt.com",
        );
    });

    it("requires the account id", () => {
        expect(() => readDeploymentConfig({})).toThrow("AWS_ACCOUNT_ID");
    });
});

describe("readEnvironment", () => {
    it("reports every invalid variable at once", () => {
        const invalidEnvironment = { AWS_ACCOUNT_ID: "12345", AWS_REGION: "mars-1", DEPLOYMENT_STAGE: "dev", DOMAIN_NAME: "not a host" };

        expect(() => readEnvironment(invalidEnvironment)).toThrow(
            /AWS_ACCOUNT_ID[\s\S]*AWS_REGION[\s\S]*DEPLOYMENT_STAGE[\s\S]*DOMAIN_NAME/,
        );
    });

    it("rejects a domain outside the hosted zone", () => {
        expect(() => readEnvironment({ AWS_ACCOUNT_ID: account, DOMAIN_NAME: "agents.example.org" })).toThrow(
            "DOMAIN_NAME: must be HOSTED_ZONE_NAME or a subdomain of it",
        );
    });

    it("accepts the zone apex as the domain", () => {
        expect(readEnvironment({ AWS_ACCOUNT_ID: account, DOMAIN_NAME: "gavindhondt.com" }).DOMAIN_NAME).toBe("gavindhondt.com");
    });
});

describe(".env.example", () => {
    it("lists exactly the variables the app reads", () => {
        const listed = readFileSync(new URL("../../../.env.example", import.meta.url), "utf8")
            .split("\n")
            .map((line) => /^#?\s*([A-Z][A-Z0-9_]+)=/.exec(line)?.[1])
            .filter((name): name is string => name !== undefined);

        expect(listed.toSorted()).toEqual(Object.values(ENVIRONMENT_VARIABLE).toSorted());
    });
});
