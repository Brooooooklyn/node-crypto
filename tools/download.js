const fs = require('fs')
const request = require('request')
const child_process = require('child_process')
const FILE_NAME = require('./filename')

const HOST = 'https://storage.googleapis.com/storage.lynvv.xyz'
const distFileName = 'native.node'

request
  .get(`${HOST}/${FILE_NAME}`)
  .pipe(fs.createWriteStream(distFileName))
  .on('error', (e) => {
    console.error(e)
    fs.unlinkSync(distFileName)
  })
  .on('close', () => {
    const size = fs.statSync(distFileName).size
    if (size < 10000) {
      console.error('Download prebuilt binary fail, fallback to build')
      fs.unlinkSync(distFileName)
      child_process.execSync('npm run build', {
        stdio: [0, 1, 2],
      })
    } else {
      console.log('download prebuilt file success ', FILE_NAME)
    }
  })
