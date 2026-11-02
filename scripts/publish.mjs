// Publishes any crate whose current version isn't on crates.io yet, then tags
// the release. `tiberqueries_derive` must go first since `tiberqueries`
// depends on it. Safe to re-run: already published versions are skipped.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

const crates = ["tiberqueries_derive", "tiberqueries"];

const run = (cmd, args) => execFileSync(cmd, args, { stdio: "inherit" });

async function isPublished(name, version) {
  const res = await fetch(`https://crates.io/api/v1/crates/${name}/${version}`, {
    headers: { "User-Agent": "tiberqueries-release (github.com/ieedan/tiberqueries)" },
  });
  if (res.status === 404) return false;
  if (!res.ok) throw new Error(`crates.io returned ${res.status} for ${name}@${version}`);
  return true;
}

for (const crate of crates) {
  const { version } = JSON.parse(readFileSync(`${crate}/package.json`, "utf8"));

  if (await isPublished(crate, version)) {
    console.log(`${crate}@${version} is already published, skipping`);
    continue;
  }

  run("cargo", ["publish", "--package", crate]);
}

// Creates git tags (e.g. tiberqueries@0.1.4) for versions without one. The
// changesets action pushes them and creates the GitHub releases.
run("pnpm", ["exec", "changeset", "tag"]);
