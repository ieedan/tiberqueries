// Copies the versions bumped by `changeset version` (package.json) into each
// crate's Cargo.toml, and keeps the tiberqueries -> tiberqueries_derive
// dependency requirement in sync so both crates always publish together.
import { readFileSync, writeFileSync } from "node:fs";

const crates = ["tiberqueries", "tiberqueries_derive"];

const read = (path) => readFileSync(path, "utf8");

for (const crate of crates) {
  const { version } = JSON.parse(read(`${crate}/package.json`));
  const path = `${crate}/Cargo.toml`;

  // Only touch the first `version = "..."` line, which is the [package] one.
  let toml = read(path).replace(/^version = ".*"$/m, `version = "${version}"`);

  if (crate === "tiberqueries") {
    toml = toml.replace(
      /(tiberqueries_derive = \{ version = ")[^"]*(")/,
      `$1${version}$2`
    );
  }

  writeFileSync(path, toml);
  console.log(`${path} -> ${version}`);
}
