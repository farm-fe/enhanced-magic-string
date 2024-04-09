import { execSync } from "child_process";

execSync("npm config set access public", { stdio: "inherit" });
// publish node packages
execSync("npx changeset publish", { stdio: "inherit" });
