import { AwsRegion } from "../model/region";
import { Stage } from "../model/stage";
import { ENVIRONMENT_VARIABLE } from "./variables";
import { z } from "zod";

/** A value left blank (`NAME=`) is the same as one that is absent, so `.env.example` can list every variable. */
function treatBlankAsMissing(value: unknown): unknown {
    if (typeof value !== "string") {
        return value;
    }

    return value.trim() === "" ? undefined : value.trim();
}

const hostname = z
    .string()
    .toLowerCase()
    .regex(/^(?=.{1,253}$)([a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,}$/, "must be a hostname such as example.com");

export const awsAccountIdSchema = z
    .string()
    .regex(/^\d{12}$/, "must be a twelve-digit AWS account id")
    .brand<"AwsAccountId">();
export const domainNameSchema = hostname.brand<"DomainName">();
export const hostedZoneNameSchema = hostname.brand<"HostedZoneName">();

export type AwsAccountId = z.infer<typeof awsAccountIdSchema>;
export type DomainName = z.infer<typeof domainNameSchema>;
export type HostedZoneName = z.infer<typeof hostedZoneNameSchema>;

export const environmentSchema = z
    .object({
        [ENVIRONMENT_VARIABLE.awsAccountId]: z.preprocess(treatBlankAsMissing, awsAccountIdSchema),
        [ENVIRONMENT_VARIABLE.awsRegion]: z.preprocess(treatBlankAsMissing, z.enum(AwsRegion).default(AwsRegion.US_EAST_1)),
        [ENVIRONMENT_VARIABLE.deploymentStage]: z.preprocess(treatBlankAsMissing, z.enum(Stage).default(Stage.PROD)),
        [ENVIRONMENT_VARIABLE.domainName]: z.preprocess(
            treatBlankAsMissing,
            domainNameSchema.default("agents.gavindhondt.com" as DomainName),
        ),
        [ENVIRONMENT_VARIABLE.hostedZoneName]: z.preprocess(
            treatBlankAsMissing,
            hostedZoneNameSchema.default("gavindhondt.com" as HostedZoneName),
        ),
    })
    .refine(
        (values) => {
            const domain: string = values[ENVIRONMENT_VARIABLE.domainName];
            const zone: string = values[ENVIRONMENT_VARIABLE.hostedZoneName];

            return domain === zone || domain.endsWith(`.${zone}`);
        },
        { path: [ENVIRONMENT_VARIABLE.domainName], message: `must be ${ENVIRONMENT_VARIABLE.hostedZoneName} or a subdomain of it` },
    );

export type ParsedEnvironment = z.infer<typeof environmentSchema>;
