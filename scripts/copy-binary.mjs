import { copyFile, mkdir } from 'node:fs/promises';
import { platform } from 'node:os';

await mkdir('dist/bin', { recursive: true });
const suffix = platform() === 'win32' ? '.exe' : '';
await copyFile(`target/release/code-path-lens${suffix}`, `dist/bin/code-path-lens${suffix}`);
