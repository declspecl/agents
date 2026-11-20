import { environmentSchema, type ParsedEnvironment } from "./schema";
import type { RuntimeEnvironment } from "./variables";

/** Parses the whole environment at once, so one failed run names every bad variable and not just the first. */
export function readEnvironment(environment: RuntimeEnvironment): ParsedEnvironment {
    const result = environmentSchema.safeParse(environment);
    if (result.success) {
        return result.data;
    }

    const problems = result.error.issues.map((issue) => `  ${issue.path.join(".") || "environment"}: ${issue.message}`);

    throw new Error(`Invalid environment:\n${problems.join("\n")}`);
}
