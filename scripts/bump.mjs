import { execSync } from "child_process";

execSync('pnpm run build', { stdio: 'inherit' });
execSync('npx changeset version', { stdio: 'inherit' });
execSync('pnpm install --no-frozen-lockfile', { stdio: 'inherit' });
