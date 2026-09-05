import { spawnSync } from 'node:child_process';

const claimArgs = process.argv.slice(2);

function run(command, args) {
  const result = spawnSync(command, args, { stdio: 'inherit' });
  if (result.status !== 0) process.exit(result.status ?? 1);
}

run('cargo', ['test']);
run('cargo', ['build']);
run('npm', ['run', 'build:site']);
run(process.execPath, ['site/test.mjs']);
run(process.execPath, ['tests/claims.mjs', ...claimArgs]);
