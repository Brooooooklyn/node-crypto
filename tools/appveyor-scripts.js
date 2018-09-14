const { execSync } = require('child_process')
const filename = require('./filename')

execSync(`mkdir -p dist && cp native/index.node dist/${filename}`, {
  env: process.env,
  stdio: [0, 1, 2],
})
