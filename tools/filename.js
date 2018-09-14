const semver = require('semver')
const os = require('os')
const version = require('../package.json').version

const MAJOR_VERSION = semver.parse(process.version).major
module.exports = `crypto-node-${ process.platform }-` +
  `${ process.env.ARC || os.arch() }-` +
  `${ MAJOR_VERSION }-${ version }.node`
