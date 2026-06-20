import { spawnSync } from "node:child_process";

const result = spawnSync("docker", ["info"], {
  encoding: "utf8",
  stdio: "pipe"
});

if (result.status === 0) {
  process.exit(0);
}

const commandMissing = result.error?.code === "ENOENT";

if (commandMissing) {
  console.error("Docker is not installed or is not on PATH.");
  console.error("Install Docker Desktop, start it, then run `pnpm dev` again.");
} else {
  console.error("Docker is not running.");
  console.error("Start Docker Desktop, wait until it finishes starting, then run `pnpm dev` again.");
}

process.exit(1);
