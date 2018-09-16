const fs = require('fs')
const filename = require('./filename')

try {
  if (!fs.existsSync('./dist')) {
    fs.mkdirSync('./dist')
  }

  fs.copyFileSync('./native.node', `./dist/${filename}`)
} catch (e) {
  console.error(e)
  process.exit(101)
}
