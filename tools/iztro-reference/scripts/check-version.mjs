import { astro } from "iztro";
import { createRequire } from "node:module";

const TARGET_VERSION = "2.5.8";

const require = createRequire(import.meta.url);
const { version } = require("iztro/package.json");

if (!astro || typeof astro.byLunar !== "function") {
  throw new Error("Installed iztro package does not expose astro.byLunar");
}

if (version !== TARGET_VERSION) {
  throw new Error(`Expected iztro ${TARGET_VERSION}, found ${version}`);
}

console.log(`iztro ${version} reference workspace ready`);
