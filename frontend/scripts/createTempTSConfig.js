// This is needed for lint-staged
// as tsc doesn't support having files as input
// and specifying a tsconfig.json at the same time.

const fs = require('fs');
const path = require('path');
const {spawn} = require('child_process');

const json = JSON.stringify(
  {
    extends: './tsconfig.json',
    include: process.argv.slice(2),
  },
  null,
  4,
);

fs.writeFile(path.join(process.cwd(), '.tsconfig-lint.json'), json, err => {
  if (err) {
    console.log(`Something bad happened: ${err}`);
    return process.exit(1);
  }
});

const tsc = spawn('npx', ['tsc', '--project', '.tsconfig-lint.json']);

tsc.stdout.on('data', data => {
  console.log(`${data}`);
});

tsc.stderr.on('data', data => {
  console.error(`${data}`);
});

tsc.on('close', code => {
  console.log(`child process exited with code ${code}`);
  return process.exit(code);
});
