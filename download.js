const fs = require('fs')
const version = require('./package.json').version
const https = require('https')
const semver = require('semver')
const child_process = require('child_process')

const MAJOR_VERSION = semver.parse(process.version).major
const HOST = 'https://storage.googleapis.com/storage.lynvv.xyz'

https.get(`${HOST}/crypto-node-${ process.platform }-${ MAJOR_VERSION }-${ version }`).pipe(
  fs.createWriteStream('index.node')
)
  .on('error', (e) => {
    console.error(e)
    fs.unlinkSync('index.node')
  })
  .on('close', () => {
    const size = fs.statSync('index.node').size
    if (!size) {
      console.error('Download prebuilt binary fail, fallback to build')
      fs.unlinkSync('index.node')
      child_process.execSync('npm run build', {
        stdio: [0, 1, 2]
      })
      fs.copyFileSync('./native/index.node', 'index.node')
    }
  })
