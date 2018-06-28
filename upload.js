const fs = require('fs')
const Storage = require('@google-cloud/storage')
const semver = require('semver')
const version = require('./package.json').version

const projectId = 'cloud-9-183315'

const storage = new Storage({ projectId })
const MAJOR_VERSION = semver.parse(process.version).major
const FILE_NAME = `crypto-node-linux-${ MAJOR_VERSION }-${ version }`

storage
  .bucket('storage.lynvv.xyz')
  .upload('./native/index.node', {
    public: true,
    destination: FILE_NAME,
    resumable: false,
  }, function (err) {
    if (err) {
      console.error(`ERROR: fail to upload ${ FILE_NAME }`, err)
      return
    }
    console.log(`${ FILE_NAME } uploaded.`)
  })
