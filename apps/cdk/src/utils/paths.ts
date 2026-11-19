import path from "node:path";
import { fileURLToPath } from "node:url";

export const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");

export function resolveRepositoryPath(...segments: string[]): string {
    return path.resolve(repositoryRoot, ...segments);
}
