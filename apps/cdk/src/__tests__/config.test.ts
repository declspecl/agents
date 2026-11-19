import { readDeploymentConfig } from "../config";
import { describe, expect, it } from "vitest";

describe("readDeploymentConfig", () => {
    it("defaults the domain and region", () => {
        const config = readDeploymentConfig({ AWS_ACCOUNT_ID: "123456789012" });

        expect(config.domainName).toBe("agents.gavindhondt.com");
        expect(config.hostedZoneName).toBe("gavindhondt.com");
        expect(config.env).toEqual({ account: "123456789012", region: "us-east-1" });
    });

    it("requires the account id", () => {
        expect(() => readDeploymentConfig({})).toThrow("AWS_ACCOUNT_ID must be set");
    });
});
