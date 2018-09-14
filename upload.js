const fs = require('fs')
const path = require('path')
const Storage = require('@google-cloud/storage')
const semver = require('semver')
const { execSync } = require('child_process')
const version = require('./package.json').version

const projectId = 'cloud-9-183315'

const storage = new Storage({ projectId })
const MAJOR_VERSION = semver.parse(process.version).major
const FILE_NAME = `crypto-node-${ process.platform }-${ MAJOR_VERSION }-${ version }.node`

const SECRET_FILE_PATH = path.join(process.env.HOME, 'google-secret-file.json')

const secretJson = process.env.GOOGLE_CLOUD_STORAGE_CONFIG ? JSON.parse(decodeURIComponent(process.env.GOOGLE_CLOUD_STORAGE_CONFIG)) : null
if (secretJson) {
  fs.writeFileSync(SECRET_FILE_PATH, JSON.stringify(secretJson))
  process.env.GOOGLE_APPLICATION_CREDENTIALS = SECRET_FILE_PATH
}

execSync(`mkdir -p dist && cp ./native/index.node ./dist/${FILE_NAME}`)

storage
  .bucket('storage.lynvv.xyz')
  .upload(`./dist/${FILE_NAME}`, {
    public: true,
    destination: FILE_NAME,
    resumable: false,
  }, function (err) {
    if (err) {
      console.error(`ERROR: fail to upload ${ FILE_NAME }`, err)
      process.exit(1)
    }
    console.log(`${ FILE_NAME } uploaded.`)
  })
